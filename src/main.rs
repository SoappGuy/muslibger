#![allow(unused)]
mod args_parse;
mod error;
mod processing;

use ::audiotags::{MimeType, Picture, Tag};
use log::debug;
use std::env;

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
        let tag = Tag::new().read_from_path(path).unwrap();
        debug!("{}", tag.title().unwrap());
        true
    })?;

    Ok(())
}
