use crate::config::Config;
use crate::error::MakerResult;

mod config;
mod error;

fn main() -> MakerResult<()> {
    let config = Config::read_config()?;
    Ok(())
}
