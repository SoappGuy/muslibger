use std::{io, path::PathBuf};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("IO error occured")]
    IO(#[from] io::Error),

    #[error("Path `{0}` is not exist")]
    PathNotExist(PathBuf),

    #[error("Invalid destination path specified")]
    Destination,

    #[error("Manual interupt")]
    Interrupt,

    #[error("Unknown error")]
    Unknown,
}
