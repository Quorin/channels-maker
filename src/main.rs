#[macro_use]
extern crate lazy_static;

use clap::{AppSettings, Clap};

use crate::config::Config;
use crate::error::MakerResult;
use crate::maker::Maker;

mod config;
mod error;
mod maker;

#[derive(Clap)]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opts {
    #[clap(short, long, about = "Delete directories and files that are not whitelisted")]
    force: bool,
}

fn main() -> MakerResult<()> {
    let opts: Opts = Opts::parse();
    let config = Config::read_config()?;
    let maker = Maker::new(config)?;
    maker.check_current_directory(opts.force)?;
    Ok(())
}
