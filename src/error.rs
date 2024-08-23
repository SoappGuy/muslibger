use indoc::indoc;
use lofty::error::LoftyError;
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

    #[error("{}", indoc! {"
        Your music library manager

        Usage: muslibger [OPTIONS] [PATHS...]

        Options:
          -h, --help                Show this help message and exit
          -d, --destination=PATH    Specify the destination directory for processed files

        For options that require PATH, if no path/paths are provided, the current working directory will be used.
    "})]
    HelpInterrupt,

    #[error("Tag Parsing error occured")]
    Parsing(#[from] LoftyError),

    #[error("Manual interupt")]
    Interrupt,

    #[error("Unknown error")]
    Unknown,
}
