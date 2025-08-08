use crate::error::Error;
use eproj::SpatialReferenceIdentifier;
use etiles::Tileset;
use etiles::io::EtilesWriter;
use std::fs;
use std::path::Path;
use std::time::Instant;
use tracing::info;

pub fn run(
    input_path: impl AsRef<Path>,
    output_directory_path: impl AsRef<Path>,
    maximum_points_per_octant: u64,
    source_crs: SpatialReferenceIdentifier,
    seed_number: Option<u64>,
) -> Result<(), Error> {
    info!("Start reading point cloud");
    let now = Instant::now();
    let auto_reader = epoint::io::AutoReader::from_path(input_path)?;
    let point_cloud = auto_reader.finish()?;
    info!("Read point cloud in {}s", now.elapsed().as_secs());

    let tileset = Tileset::from_point_cloud(
        point_cloud,
        source_crs,
        maximum_points_per_octant,
        seed_number,
    )?;

    if let Some(parent) = output_directory_path.as_ref().parent() {
        fs::create_dir_all(parent)?;
    }
    info!(
        "Start writing tileset to: {}",
        output_directory_path.as_ref().display()
    );
    let writer = EtilesWriter::from_path(output_directory_path)?;
    writer.finish(&tileset)?;
    info!("Completed");

    Ok(())
}
