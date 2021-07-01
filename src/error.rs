use std::ffi::OsString;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum MakerError {
    #[error("config.json file not found")]
    NotFoundConfig,
    #[error("config.json file is malformed")]
    UnprocessableConfig(#[from] serde_json::Error),
    #[error("cannot read file or directory")]
    Io(#[from] std::io::Error),
    #[error("before execution you have to delete: {0:?}")]
    DirectoryNotEmpty(Vec<OsString>),
}

pub type MakerResult<T> = Result<T, MakerError>;