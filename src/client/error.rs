use thiserror::Error;

#[derive(Error, Debug)]
pub enum ClientError {
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("API error: {0}")]
    Api(String),

    #[error("Model not found: {0}")]
    ModelNotFound(String),

    #[error("Connection refused")]
    ConnectionRefused,

    #[error("Stream error: {0}")]
    StreamError(String),

    #[error("Parse error: {0}")]
    ParseError(#[from] serde_json::Error),

    #[error("Other error: {0}")]
    Other(#[from] anyhow::Error),
}

pub type Result<T> = std::result::Result<T, ClientError>;
