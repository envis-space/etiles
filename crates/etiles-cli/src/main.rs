mod cli;
mod commands;
mod error;

use crate::cli::{Cli, Commands};
use anyhow::Result;
use clap::Parser;
use eproj::SpatialReferenceIdentifier;

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let cli = Cli::parse();

    match &cli.command {
        Commands::ConvertPointCloud {
            input_path,
            output_path,
            maximum_points_per_octant,
            source_crs,
            no_shuffle,
            seed,
        } => {
            if !input_path.exists() {
                anyhow::bail!("input path does not exist: {}", input_path.display());
            }
            if input_path.is_file() && epoint::io::PointCloudFormat::from_path(input_path).is_none()
            {
                anyhow::bail!("unrecognized point cloud format: {}", input_path.display());
            }
            if output_path.extension().and_then(|e| e.to_str()) != Some("tar") {
                anyhow::bail!(
                    "output path must have a .tar extension: {}",
                    output_path.display()
                );
            }

            let source_crs = SpatialReferenceIdentifier::from_code(*source_crs)?;
            let seed_number = if *no_shuffle { None } else { Some(*seed) };

            commands::convert_point_cloud::run(
                input_path,
                output_path,
                *maximum_points_per_octant,
                source_crs,
                seed_number,
            )?;
        }
    };

    Ok(())
}
