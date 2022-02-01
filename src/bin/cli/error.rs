use std::{error, io};
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("{0}")]
    Lib(#[from] lib_pendulum_launch::error::Error),
    #[error("Must provide valid path")]
    InvalidBinary,
    #[error("Must provide valid path")]
    InvalidDirectory,
    #[error("Must provide launcher")]
    LaunchCommand,
    #[error("{0}")]
    Io(#[from] io::Error),
    #[error("{0}")]
    Other(#[from] Box<dyn error::Error>),
}
