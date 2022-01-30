use serde::{de, ser};
use std::{error, fmt, io, result};
use thiserror::Error;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("{0}")]
    Io(#[from] io::Error),
    #[error("{0}")]
    Signal(#[from] ctrlc::Error),
    #[error("{0}")]
    Serde(#[from] SerdeError),
    #[error("{0}")]
    Other(#[from] Box<dyn error::Error>),
}

#[derive(Debug, Error)]
pub enum SerdeError {
    #[error("{0}")]
    Serialize(String),
    #[error("{0}")]
    Deserialize(String),
}

impl ser::Error for SerdeError {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        SerdeError::Serialize(format!("Failed to serialize: {msg}"))
    }
}

impl de::Error for SerdeError {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        SerdeError::Deserialize(format!("Failed to deserialize: {msg}"))
    }
}
