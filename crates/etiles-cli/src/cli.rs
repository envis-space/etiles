use clap::{Parser, Subcommand, ValueHint};
use std::path::PathBuf;

#[derive(Parser)]
#[clap(author, version, about, long_about = None, propagate_version = true)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Convert point cloud files to 3D Tiles
    ConvertPointCloud {
        /// Path to a point cloud file or a directory containing point cloud files to be combined.
        /// Supported formats: LAS, LAZ, E57, XYZ, XYZ+Zstandard.
        #[clap(long, value_hint = ValueHint::AnyPath, value_name = "PATH")]
        input_path: PathBuf,

        /// Path to a TAR archive where the derived 3D Tiles will be stored. Extension must be .tar.
        #[clap(long, value_hint = ValueHint::FilePath, value_name = "PATH")]
        output_path: PathBuf,

        /// Maximum number of points stored per octree node.
        /// Lower values produce more, smaller tiles;
        /// higher values produce fewer, larger tiles.
        #[clap(long, default_value_t = 100000, value_name = "N")]
        maximum_points_per_octant: u64,

        /// EPSG code of the coordinate reference system of the source point cloud
        /// (e.g. 25832 for ETRS89 / UTM zone 32N).
        #[clap(long, value_name = "EPSG_CODE")]
        source_crs: u32,

        /// Disable random shuffling of points before building the octree.
        /// Shuffling is on by default and improves spatial distribution across tiles.
        #[clap(long)]
        no_shuffle: bool,

        /// Seed for the random shuffle, ensuring reproducible output.
        /// Only used when --no-shuffle is not set.
        #[clap(long, default_value_t = 1, value_name = "SEED")]
        seed: u64,
    },
}
