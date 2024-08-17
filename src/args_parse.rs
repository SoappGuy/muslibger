use crate::error::Error;
use std::{env, path::PathBuf};

use colored::Colorize;
use log::{error, info, warn};

pub fn parse(args: Vec<String>) -> Result<Args, Error> {
    let current_dir = env::current_dir()?;

    let mut paths = Vec::new();
    let mut destination_root = current_dir.clone();

    for arg in &args[1..] {
        info!("processing arg: `{:?}`", arg);

        let mut parse = arg.split("=").fuse();

        let first_part = parse.next();
        let second_part = parse.next();

        match first_part {
            Some("-d") | Some("--destination") => {
                info!("setting new {} path", "destination".cyan());

                match second_part {
                    Some(path) => {
                        let path = PathBuf::from(path);

                        if path.is_dir() {
                            info!("new {} path is set", "destination".cyan());
                            destination_root = path;
                        } else {
                            error!("provided {} path is invalid", "destination".cyan());
                            return Err(Error::Destination);
                        }
                    }
                    None => {
                        error!("provided {} path is invalid", "destination".cyan());
                        return Err(Error::Destination);
                    }
                }
            }
            Some("-h") | Some("--help") => {
                info!("parsed help request");
                return Err(Error::HelpInterrupt);
            }
            Some(path) => {
                info!("adding new {} to processing queue", "path".green());

                let path = PathBuf::from(path);
                if path.exists() {
                    paths.push(path);
                } else {
                    warn!(
                        "skipping nonexisting {}:\t{:?}",
                        "file".blue(),
                        path.file_name().unwrap()
                    );
                }
            }
            None => {
                return Err(Error::Unknown);
            }
        }
    }

    if paths.is_empty() {
        warn!(
            "no {} was added to precessing queue, adding {} as path to process",
            "paths".green(),
            "cwd".green()
        );
        paths.push(current_dir);
    }

    Ok(Args {
        paths_to_process: paths,
        output_dir_root: destination_root,
    })
}

#[derive(Debug)]
pub struct Args {
    pub output_dir_root: PathBuf,
    pub paths_to_process: Vec<PathBuf>,
}

impl Args {
    pub fn new() -> Self {
        let output_dir_root = match env::current_dir() {
            Ok(dir) => dir,
            Err(_) => PathBuf::new(),
        };

        let paths_to_process = Vec::new();

        Self {
            output_dir_root,
            paths_to_process,
        }
    }

    pub fn set_root(&mut self, root: PathBuf) {
        self.output_dir_root = root;
    }

    pub fn add_path(&mut self, path: PathBuf) {
        self.paths_to_process.push(path);
    }
}
