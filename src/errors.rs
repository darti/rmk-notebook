use std::io;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),

    #[error("invalid metadata: {0}")]
    InvalidFile(#[from] io::Error),

    #[error("invalid path: {0}")]
    InvalidPath(String),

    #[error(transparent)]
    InvalidMetadata(#[from] serde_json::Error),

    #[error("Unsupported version string: {0}")]
    UnsupportedVersion(String),

    #[error("Unknown brush type: {0}")]
    UnknownBrush(i32),

    #[error("Unknown color: {0}")]
    UnknownColor(i32),

    #[error("Invalid Segment index: {0}")]
    InvalidSegmentIndex(usize),
}

pub type Result<T> = std::result::Result<T, Error>;
