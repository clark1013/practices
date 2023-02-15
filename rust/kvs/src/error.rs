use std::{fmt::Display, io, string};

#[derive(Debug)]
pub enum KvsError {
    KeyNotFound,
    Io,
    Json,
    Sled,
    Utf8,
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

impl From<sled::Error> for KvsError {
    fn from(_: sled::Error) -> Self {
        KvsError::Sled
    }
}

impl From<string::FromUtf8Error> for KvsError {
    fn from(_: string::FromUtf8Error) -> Self {
        KvsError::Utf8
    }
}

pub type Result<T> = std::result::Result<T, KvsError>;

impl Display for KvsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            KvsError::KeyNotFound => write!(f, "Key not found"),
            KvsError::Io => write!(f, "Io error"),
            KvsError::Json => write!(f, "Json error"),
            KvsError::Sled => write!(f, "Sled error"),
            KvsError::Utf8 => write!(f, "Utf8 error"),
        }
    }
}
