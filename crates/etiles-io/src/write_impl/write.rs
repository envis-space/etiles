use crate::documents::bounding_volume::BoundingVolume;
use crate::documents::content::Content;
use crate::documents::implicit_tiling::{ImplicitTiling, SubdivisionScheme, Subtrees};
use crate::documents::tile::{Refinement, Tile};
use crate::documents::tileset::TilesetDocument;
use crate::write_impl::write_subtree::write_subtree;
use crate::{EncodableContent, Error, FILE_NAME_TILESET_JSON};
use chrono::Utc;
use ecoord::octree::{OctantIndex, Octree};
use etiles_core::{BoundingCube, Tileset, Vertex};
use std::collections::HashMap;
use std::io::{Cursor, Write};
use std::path::PathBuf;

pub fn write<W: Write>(
    writer: W,
    tileset: &Tileset,
    content_directory_path: PathBuf,
    subtrees_directory_path: PathBuf,
    levels_per_subtree: usize,
) -> Result<(), Error> {
    let mut archive_builder = tar::Builder::new(writer);

    //info!("Writing write_tileset_json");
    let tileset_document_buffer = write_tileset_json(
        tileset,
        &content_directory_path,
        &subtrees_directory_path,
        levels_per_subtree,
    )?;
    //info!("Writing append_data");
    archive_builder.append_data(
        &mut create_archive_header(tileset_document_buffer.len(), None),
        FILE_NAME_TILESET_JSON,
        Cursor::new(tileset_document_buffer),
    )?;

    //info!("Writing write_subtree_info");
    let subtree_binaries = write_subtree_info(levels_per_subtree, &tileset.tiled_content)?;
    for (current_subtree_binary_name, current_data_buffer) in subtree_binaries {
        archive_builder.append_data(
            &mut create_archive_header(current_data_buffer.len(), None),
            subtrees_directory_path.join(current_subtree_binary_name),
            Cursor::new(current_data_buffer),
        )?;
    }

    //info!("Writing tileset.tiled_content");
    let encoded_content_tiles: HashMap<String, Vec<u8>> = tileset
        .tiled_content
        .cell_indices()
        .into_iter()
        .map(|x| {
            //info!("Get cell content: {}", x);
            let cell_content = tileset.tiled_content.cell(x).expect("must be contained");

            //info!("Encode cell content: {}", x);
            (derive_content_filename(&x), cell_content.encode().unwrap())
        })
        .collect();
    //info!(
    //    "Writing encoded_content_tiles: {}",
    //    tileset.tiled_content.cell_count()
    //);
    for (current_encoded_content_tile_name, current_data_buffer) in encoded_content_tiles {
        archive_builder.append_data(
            &mut create_archive_header(current_data_buffer.len(), None),
            content_directory_path.join(current_encoded_content_tile_name),
            Cursor::new(current_data_buffer),
        )?;
    }
    //info!("Finished encoded_content_tiles");

    Ok(())
}

pub fn write_subtree_info(
    levels_per_subtree: usize,
    content_octree: &Octree<Vertex>,
) -> Result<HashMap<String, Vec<u8>>, Error> {
    let mut subtree_infos = HashMap::new();
    let max_occupied_level = if let Some(v) = content_octree.get_max_occupied_level() {
        v
    } else {
        return Ok(subtree_infos);
    };

    let occupied_octant_indices: Vec<OctantIndex> = (0..=max_occupied_level)
        .step_by(levels_per_subtree)
        .flat_map(|l| {
            content_octree
                .occupancy_graph()
                .get_occupied_cell_indices_of_level(l)
        })
        .collect();

    for current_occupied_octant_index in occupied_octant_indices {
        let mut subtree_info_buffer: Vec<u8> = Vec::new();
        write_subtree(
            &mut subtree_info_buffer,
            current_occupied_octant_index,
            levels_per_subtree,
            content_octree,
        )
        .expect("should work");

        let file_name = format!(
            "{}__{}_{}_{}.subtree",
            current_occupied_octant_index.level,
            current_occupied_octant_index.x,
            current_occupied_octant_index.y,
            current_occupied_octant_index.z
        );
        subtree_infos.insert(file_name, subtree_info_buffer);
    }

    Ok(subtree_infos)
}

pub fn write_tileset_json(
    tileset: &Tileset,
    content_directory_path: &PathBuf,
    subtrees_directory_path: &PathBuf,
    levels_per_subtree: usize,
) -> Result<Vec<u8>, Error> {
    let tile = derive_implicit_tile_from_content_octree(
        OctantIndex::origin(),
        content_directory_path,
        subtrees_directory_path,
        levels_per_subtree,
        tileset.geometric_error,
        &tileset.tiled_content,
    )?;

    // info!("diagonal: {root_geometric_error}");
    let tileset_document = TilesetDocument::new(
        tile,
        Some(tileset.root_transform),
        tileset.root_geometric_error,
    );
    let mut tileset_document_buffer: Vec<u8> = Vec::new();
    serde_json::to_writer_pretty(&mut tileset_document_buffer, &tileset_document)?;
    Ok(tileset_document_buffer)
}

pub fn derive_implicit_tile_from_content_octree(
    index: OctantIndex,
    content_directory_path: &PathBuf,
    subtree_directory_path: &PathBuf,
    levels_per_subtree: usize,
    geometric_error: f64,
    content_octree: &Octree<Vertex>,
) -> Result<Tile, Error> {
    let current_bounding_cube = content_octree.bounds().get_octant_bounding_cube(index);
    let bounding_cube = BoundingCube::from_axis_aligned_bounding_cube(&current_bounding_cube);

    let tile = Tile {
        geometric_error,
        content: Content {
            uri: content_directory_path
                .clone()
                .join("pc_{level}__{x}_{y}_{z}.glb")
                .to_str()
                .unwrap()
                .to_string(),
        },
        bounding_volume: BoundingVolume::Box(bounding_cube.bounding_array()),
        children: vec![],
        transform: None,
        refine: Some(Refinement::Add),
        implicit_tiling: Some(ImplicitTiling {
            subdivision_scheme: SubdivisionScheme::Octree,
            subtree_levels: levels_per_subtree as u16,
            available_levels: content_octree.get_max_occupied_level().unwrap_or_default() as u16,
            subtrees: Subtrees {
                uri: subtree_directory_path
                    .clone()
                    .join("{level}__{x}_{y}_{z}.subtree")
                    .to_str()
                    .unwrap()
                    .to_string(),
            },
        }),
    };

    Ok(tile)
}

pub fn derive_content_filename(index: &OctantIndex) -> String {
    format!(
        "pc_{}__{}_{}_{}.glb",
        index.level, index.x, index.y, index.z
    )
}

fn create_archive_header(size: usize, time: Option<chrono::DateTime<Utc>>) -> tar::Header {
    let mut header = tar::Header::new_gnu();
    header.set_size(size as u64);
    header.set_mode(0o664);
    if let Some(time) = time {
        header.set_mtime(time.timestamp() as u64);
    }
    header.set_cksum();

    header
}
