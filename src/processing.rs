use colored::Colorize;
use log::{debug, info};
use std::{
    collections::HashSet,
    fs,
    path::{Path, PathBuf},
};

use crate::error;

fn process_path(
    path: PathBuf,
    call_on_file: fn(&Path) -> bool,
) -> Result<HashSet<PathBuf>, error::Error> {
    let mut processed = HashSet::new();

    if path.is_dir() {
        debug!("processing {}: \t{:?}", "dir".yellow(), path);

        for path in fs::read_dir(path)? {
            let path = path?.path();

            if path.exists() && !processed.contains(&path) {
                processed.extend(process_path(path, call_on_file)?);
            }
        }
    } else {
        debug!(
            "processing {}:\t{:?}",
            "file".blue(),
            &path.file_name().unwrap()
        );

        if call_on_file(&path) {
            processed.insert(path);
        }
    }

    Ok(processed)
}

pub fn process_paths(
    paths: &Vec<PathBuf>,
    call_on_file: fn(&Path) -> bool,
) -> Result<usize, error::Error> {
    info!(
        "collected {} entries, processing",
        paths.len().to_string().red()
    );

    let mut processed = HashSet::new();

    for path in paths {
        if !processed.contains(path) {
            processed.extend(process_path(path.to_path_buf(), call_on_file)?);
        }
    }

    let len = processed.len();
    info!("processed {} entries", len.to_string().red());
    Ok(len)
}
