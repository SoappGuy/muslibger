#![allow(unused)]
mod args_parse;
mod error;

use std::env;
use std::fs;
use std::path::Path;
use std::path::PathBuf;

use colored::Colorize;
use error::Error;
use log::{debug, info, warn};

fn main() -> Result<(), error::Error> {
    env_logger::builder().format_timestamp_secs().init();

    let cmd_args: Vec<String> = env::args().collect();

    let parsed_args = match args_parse::parse(cmd_args) {
        Ok(args) => args,
        Err(err) => {
            println!("{err}");
            return Ok(());
        }
    };

    process_paths(&parsed_args.paths_to_process, |_path| {
        // info!("{path:?}");
        true
    })?;

    Ok(())
}

fn process_path(path: &Path, call_on_file: fn(&Path) -> bool) -> Result<i32, Error> {
    let mut processed = 0;

    if path.is_dir() {
        debug!("processing {}: \t{:?}", "dir".yellow(), path);

        for path in fs::read_dir(path)? {
            let path = path?.path();

            if path.exists() {
                if path.is_dir() {
                    processed += process_path(&path, call_on_file)?;
                } else {
                    debug!(
                        "processing {}:\t{:?}",
                        "file".blue(),
                        &path.file_name().unwrap()
                    );

                    processed += if call_on_file(&path) { 1 } else { 0 };
                }
            } else {
                debug!(
                    "skipping nonexisting {}: \t{:?}",
                    "file".blue(),
                    &path.file_name().unwrap()
                );
            }
        }
    } else {
        debug!(
            "processing {}:\t{:?}",
            "file".blue(),
            &path.file_name().unwrap()
        );

        processed += if call_on_file(path) { 1 } else { 0 };
    }
    Ok(processed)
}

fn process_paths(paths: &Vec<PathBuf>, call_on_file: fn(&Path) -> bool) -> Result<i32, Error> {
    info!(
        "collected {} entries, processing",
        paths.len().to_string().red()
    );

    let mut processed = 0;

    for path in paths {
        processed += process_path(path, call_on_file)?;
    }

    info!("processed {} entries", processed.to_string().red());
    Ok(processed)
}
