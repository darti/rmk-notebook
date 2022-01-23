use std::io;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),

    #[error("invalid metadata: {0}")]
    InvalidFile(#[from] io::Error),

    #[error("invalid metadata: {0}")]
    InvalidMetadata(#[from] serde_json::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
