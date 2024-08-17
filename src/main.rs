mod args_parse;
mod error;

use error::Error;

fn main() -> Result<(), error::Error> {
    env_logger::builder().format_timestamp_secs().init();

    let args = args_parse::parse()?;

    Ok(())
}
