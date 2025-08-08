use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    EtilesError(#[from] etiles_core::Error),

    #[error(transparent)]
    EcoordError(#[from] ecoord::Error),
    #[error(transparent)]
    EpointError(#[from] epoint::Error),
    #[error(transparent)]
    EpointTransformError(#[from] epoint::transform::Error),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Parsing(#[from] serde_json::Error),

    #[error("file extension `{0}` is invalid")]
    InvalidFileExtension(String),
    #[error("invalid version of major={major} and minor={minor}")]
    InvalidVersion { major: u8, minor: u8 },
    #[error("file extension is invalid")]
    NoFileExtension(),
    #[error("file extension is invalid")]
    PointDataFileNotFound(),
}
