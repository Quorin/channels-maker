#[macro_use]
extern crate lazy_static;

use crate::config::Config;
use crate::error::MakerResult;
use crate::maker::Maker;

mod config;
mod error;
mod maker;

fn main() -> MakerResult<()> {
    let config = Config::read_config()?;
    let maker = Maker::new(config)?;
    maker.check_current_directory()?;
    Ok(())
}
