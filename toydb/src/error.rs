use std;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Abort,
    Internal(String),
    Value(String)
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Abort => write!(f, "Operation Abort"),
            Self::Internal(s) | Self::Value(s) => write!(f, "{}", s),
        }
    }
}

impl std::error::Error for Error {}

impl From<Box<bincode::ErrorKind>> for Error {
    fn from(err: Box<bincode::ErrorKind>) -> Self {
        Self::Internal(err.to_string())
    }
}