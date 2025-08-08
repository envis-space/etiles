use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(author, version, about, long_about = None, propagate_version = true)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Convert point cloud files to 3D Tiles
    ConvertPointCloudToTiles {
        /// Input file path
        #[clap(long)]
        input_path: String,

        /// Output directory
        #[clap(long)]
        output_directory_path: String,

        /// Maximum number of points per octant
        #[clap(long, default_value_t = 100000)]
        maximum_points_per_octant: u64,

        /// Reference system of the source point cloud
        #[clap(long)]
        source_crs: u32,

        /// Randomly shuffle points during processing for better spatial distribution
        #[clap(long, default_value_t = true)]
        randomly_shuffle: bool,

        /// Seed value for random shuffling (ensures reproducible results)
        #[clap(long, default_value_t = 42)]
        shuffle_seed_number: u64,
    },
}
