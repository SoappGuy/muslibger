#![allow(unused)]
mod args_parse;
mod error;
mod processing;

use lofty::{
    self,
    file::TaggedFileExt,
    tag::{Accessor, ItemKey, TagExt, TagItem},
};
use log::debug;
use std::{
    env,
    path::{Path, PathBuf},
};

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

    processing::process_paths(&parsed_args.paths_to_process, |path| {
        let mut binding = lofty::read_from_path(path).unwrap();
        let tag = binding.primary_tag_mut().unwrap();

        let track_title = tag.get_string(&ItemKey::TrackTitle).unwrap_or("no_track");
        let album_title = tag.get_string(&ItemKey::AlbumTitle).unwrap_or("no_album");
        let track_number = tag.get_string(&ItemKey::TrackNumber).unwrap_or("no_number");
        let year = tag.get_string(&ItemKey::Year).unwrap_or("no_year");
        let artist_name = tag.get_string(&ItemKey::AlbumArtist).unwrap_or("no_artist");

        let root = parsed_args.output_dir_root.to_str().unwrap();
        let destination = PathBuf::from(format!(
            "{root}/processed/{artist_name}/{album_title} ({year})/{track_number} - {track_title}"
        ));

        println!("{destination:#?}");
        std::fs::create_dir_all(destination.parent().unwrap());
        std::fs::copy(path, destination);

        true
    })?;

    Ok(())
}
