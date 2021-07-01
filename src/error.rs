use thiserror::Error;

#[derive(Debug, Error)]
pub enum MakerError {
    #[error("config.json file not found")]
    NotFoundConfig,
    #[error("config.json file is malformed")]
    UnprocessableConfig(#[from] serde_json::Error),
    #[error("cannot read config.json file")]
    Read(#[from] std::io::Error),
}

pub type MakerResult<T> = Result<T, MakerError>;