use std::{error, io, result, sync::mpsc};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("{0}")]
    Signal(#[from] ctrlc::Error),
    #[error("{0}")]
    Io(#[from] io::Error),
    #[error("{0}")]
    ChannelReceive(#[from] mpsc::RecvError),
    #[error("{0}")]
    Other(#[from] Box<dyn error::Error>),
}

pub type Result<T> = result::Result<T, Error>;
