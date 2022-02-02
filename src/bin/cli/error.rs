use std::string::FromUtf8Error;
use std::{error, io};
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("{0}")]
    Lib(#[from] lib_pendulum_launch::error::Error),
    #[error("Must provide valid path")]
    InvalidPath,
    #[error("Process failed: {0:?}")]
    ProcessFailed(Vec<u8>),
    #[error("{0}")]
    ParseJson(#[from] json::Error),
    #[error("{0}")]
    FromUtf8(#[from] FromUtf8Error),
    #[error("{0}")]
    Io(#[from] io::Error),
    #[error("{0}")]
    Other(#[from] Box<dyn error::Error>),
}

// SAFETY: Errors are not shared between threads in the client
unsafe impl Sync for Error {}
