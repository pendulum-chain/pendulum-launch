use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("{0}")]
    PendulumLaunch(#[from] lib_pendulum_launch::error::Error),
    #[error("Must provide valid path to collator binary")]
    CollatorBin,
    #[error("Must provide valid path to genesis outdir")]
    Genesis,
    #[error("Must provide valid path to spec outdir")]
    Spec,
}
