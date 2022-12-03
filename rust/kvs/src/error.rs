use std::io;

#[derive(Debug)]
pub enum KvsError {
    KeyNotFound,
    Io,
    Json,
}

impl From<io::Error> for KvsError {
    fn from(_: io::Error) -> Self {
        KvsError::Io
    }
}

impl From<serde_json::Error> for KvsError {
    fn from(_: serde_json::Error) -> Self {
        KvsError::Json
    }
}

pub type Result<T> = std::result::Result<T, KvsError>;
