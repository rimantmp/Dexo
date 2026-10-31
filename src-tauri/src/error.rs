use thiserror::Error;

#[derive(Error, Debug)]
pub enum DownbyteError {
    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Database error: {0}")]
    Database(#[from] rusqlite::Error),

    #[error("URL parse error: {0}")]
    UrlParse(#[from] url::ParseError),

    #[error("Invalid server response: {0}")]
    InvalidResponse(String),

    #[error("Server does not support range requests for multi-connection download")]
    RangeNotSupported,

    #[error("Download was cancelled or paused")]
    Cancelled,

    #[error("File integrity check failed")]
    IntegrityFailure,

    #[error("Resource not found or changed on server")]
    ResourceChanged,

    #[error("Internal engine error: {0}")]
    Internal(String),
}

impl serde::Serialize for DownbyteError {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

pub type Result<T> = std::result::Result<T, DownbyteError>;

