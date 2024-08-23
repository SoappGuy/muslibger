use colored::Colorize;
use lofty::{
    self,
    file::TaggedFileExt,
    read_from_path,
    tag::{Accessor, ItemKey, TagExt, TagItem},
};

use log::{debug, error, info};
use std::{
    collections::HashSet,
    fs,
    path::{Path, PathBuf},
};

use crate::error;

fn process_path<F: Fn(&Path) -> bool>(
    path: PathBuf,
    call_on_file: &F,
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

pub fn process_paths<F: Fn(&Path) -> bool>(
    paths: &Vec<PathBuf>,
    call_on_file: F,
) -> Result<usize, error::Error> {
    info!(
        "collected {} entries, processing",
        paths.len().to_string().red()
    );

    let mut processed = HashSet::new();

    for path in paths {
        if !processed.contains(path) {
            processed.extend(process_path(path.to_path_buf(), &call_on_file)?);
        }
    }

    let len = processed.len();
    info!("processed {} entries", len.to_string().red());
    Ok(len)
}

fn call_on_path(path: &Path, pattern: &str, root: &Path) -> Result<(), error::Error> {
    let mut binding = read_from_path(path).unwrap();

    if let Some(tag) = binding.primary_tag_mut() {
        info!("Reading Tag from {}", path.to_str().unwrap());

        let track_title = tag.get_string(&ItemKey::TrackTitle).unwrap_or("no_track");
        let album_title = tag.get_string(&ItemKey::AlbumTitle).unwrap_or("no_album");
        let track_number = tag.get_string(&ItemKey::TrackNumber).unwrap_or("no_number");
        let year = tag.get_string(&ItemKey::Year).unwrap_or("no_year");
        let artist_name = tag.get_string(&ItemKey::AlbumArtist).unwrap_or("no_artist");

        let root = root.to_str().unwrap();
        let extention = path
            .extension()
            .unwrap_or_default()
            .to_str()
            .unwrap_or_default();
        let destination = PathBuf::from(format!(
            "{root}/processed/{artist_name}/{album_title} ({year})/{track_number} - {track_title}.{extention}"
        ));

        info!("Path parsed for file: {}", destination.to_str().unwrap());

        if destination.is_file() {
            info!("Creating directory tree for destination");
            std::fs::create_dir_all(destination.parent().unwrap())?;

            info!("Copying from source to destination with new filename");
            std::fs::copy(path, destination)?;
        } else {
            error!("Filename in provided pattern is invalid")
        }
    } else {
        error!("File {} have no Tag", path.to_str().unwrap())
    }

    Ok(())
}

mod tests {
    use super::*;

    #[test]
    fn path() {
        let result = process_path("./test_files/hotwax.flac".into(), &|_| true)
            .unwrap()
            .len();

        assert_eq!(result, 1);
    }

    #[test]
    fn path_dir() {
        let result = process_path("./test_files".into(), &|_| true)
            .unwrap()
            .len();

        assert_eq!(result, 2);
    }

    #[test]
    fn paths() {
        let result = process_paths(
            &vec![
                "./test_files/hotwax.flac".into(),
                "./test_files/sissyneck.flac".into(),
            ],
            |_| true,
        )
        .unwrap();

        assert_eq!(result, 2);
    }

    #[test]
    fn paths_same() {
        let result = process_paths(
            &vec![
                "./test_files/hotwax.flac".into(),
                "./test_files/hotwax.flac".into(),
            ],
            |_| true,
        )
        .unwrap();

        assert_eq!(result, 1);
    }

    #[test]
    fn paths_empty() {
        let result = process_paths(&vec![], |_| true).unwrap();

        assert_eq!(result, 0);
    }
}
