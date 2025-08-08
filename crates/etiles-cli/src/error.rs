use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    EtilesError(#[from] etiles::Error),
    #[error(transparent)]
    EtilesIoError(#[from] etiles::io::Error),

    #[error(transparent)]
    EprojError(#[from] eproj::Error),
    #[error(transparent)]
    EpointError(#[from] epoint::Error),
    #[error(transparent)]
    EpointIoError(#[from] epoint::io::Error),
    #[error(transparent)]
    EpointTransformError(#[from] epoint::transform::Error),

    #[error(transparent)]
    StdIoError(#[from] std::io::Error),
}
