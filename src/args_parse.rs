use crate::error::Error;
use std::{env, fmt::Debug, path::PathBuf};

use colored::Colorize;
use log::{error, info, warn};

pub fn parse(args: Vec<impl AsRef<str>>) -> Result<Args, Error> {
    let current_dir = env::current_dir()?;

    let mut paths = Vec::new();
    let mut destination_root = current_dir.clone();

    for arg in &args[1..] {
        let arg = arg.as_ref();
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

mod tests {
    use super::*;

    #[test]
    fn parse_no_args() {
        let cwd = env::current_dir().unwrap();
        let cmd_str = vec!["./muslibger"];

        let result = match parse(cmd_str) {
            Ok(args) => {
                args.output_dir_root == cwd
                    && args.paths_to_process.len() == 1
                    && args.paths_to_process[0] == cwd
            }
            Err(_) => false,
        };

        assert!(result);
    }

    #[test]
    fn parse_path() {
        let cwd = env::current_dir().unwrap();
        let cmd_str = vec!["./muslibger", "./"];

        let result = match parse(cmd_str) {
            Ok(args) => {
                args.output_dir_root == cwd
                    && args.paths_to_process.len() == 1
                    && args.paths_to_process[0] == PathBuf::from("./")
            }
            Err(_) => false,
        };

        assert!(result);
    }

    #[test]
    fn parse_d() {
        let cwd = env::current_dir().unwrap();
        let cmd_str = vec!["./muslibger", "-d=./"];

        let result = match parse(cmd_str) {
            Ok(args) => {
                args.output_dir_root == PathBuf::from("./")
                    && args.paths_to_process.len() == 1
                    && args.paths_to_process[0] == cwd
            }
            Err(_) => false,
        };

        assert!(result);
    }

    #[test]
    fn parse_d_err() {
        let cmd_str = vec!["./muslibger", "-d"];

        let result = match parse(cmd_str) {
            Ok(_) => false,
            Err(err) => matches!(err, Error::Destination),
        };

        assert!(result);
    }

    #[test]
    fn parse_h() {
        let cmd_str = vec!["./muslibger", "-h"];

        let result = match parse(cmd_str) {
            Ok(_) => false,
            Err(err) => matches!(err, Error::HelpInterrupt),
        };

        assert!(result);
    }

    #[test]
    fn parse_help() {
        let cmd_str = vec!["./muslibger", "--help"];

        let result = match parse(cmd_str) {
            Ok(_) => false,
            Err(err) => matches!(err, Error::HelpInterrupt),
        };

        assert!(result);
    }
}
