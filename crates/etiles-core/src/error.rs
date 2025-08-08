use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    EcoordError(#[from] ecoord::Error),
    #[error(transparent)]
    EprojError(#[from] eproj::Error),
    #[error(transparent)]
    EpointError(#[from] epoint::Error),
}
