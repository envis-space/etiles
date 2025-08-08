mod cli;
mod commands;
mod error;

use crate::cli::{Cli, Commands};
use anyhow::Result;
use clap::Parser;
use eproj::SpatialReferenceIdentifier;
use std::path::PathBuf;

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let cli = Cli::parse();

    match &cli.command {
        Commands::ConvertPointCloudToTiles {
            input_path,
            output_directory_path,
            maximum_points_per_octant,
            source_crs,
            randomly_shuffle,
            shuffle_seed_number,
        } => {
            let input_path = PathBuf::from(input_path);
            let output_directory_path = PathBuf::from(output_directory_path);
            let source_crs = SpatialReferenceIdentifier::from_code(*source_crs)?;
            let seed_number = if *randomly_shuffle {
                Some(*shuffle_seed_number)
            } else {
                None
            };

            commands::convert_point_cloud::run(
                input_path,
                output_directory_path,
                *maximum_points_per_octant,
                source_crs,
                seed_number,
            )?;
        }
    };

    Ok(())
}
