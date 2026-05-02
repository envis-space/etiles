use crate::error::Error;
use epoint::transform::merge;
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
    let point_cloud = if input_path.as_ref().is_dir() {
        read_point_clouds_from_directory(input_path)?
    } else {
        info!("Start reading point cloud file");
        let now = Instant::now();
        let auto_reader = epoint::io::AutoReader::from_path(input_path)?;
        let point_cloud = auto_reader.finish()?;
        info!("Read point cloud in {}s", now.elapsed().as_secs());
        point_cloud
    };

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

fn read_point_clouds_from_directory(
    input_path: impl AsRef<Path>,
) -> Result<epoint::PointCloud, Error> {
    info!("Start reading point cloud directory");
    let mut point_cloud_file_count = 0;
    let now = Instant::now();

    let mut point_clouds = Vec::new();
    for entry in fs::read_dir(input_path)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() && epoint::io::PointCloudFormat::from_path(&path).is_some() {
            let auto_reader = epoint::io::AutoReader::from_path(&path)?;
            let point_cloud = auto_reader.finish()?;
            point_clouds.push(point_cloud);
            point_cloud_file_count += 1;
        }
    }

    let combined_point_cloud = merge(point_clouds)?;
    info!(
        "Read {} point cloud files in {}s",
        point_cloud_file_count,
        now.elapsed().as_secs()
    );

    Ok(combined_point_cloud)
}
