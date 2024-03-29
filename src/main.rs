#[macro_use]
extern crate lazy_static;

use clap::{AppSettings, Clap};

use crate::config::Config;
use crate::maker::Maker;

mod config;
mod maker;

#[derive(Clap)]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opts {
    #[clap(
        short,
        long,
        about = "Delete directories and files that are not whitelisted"
    )]
    force: bool,
}

pub(crate) const CONFIG_FILE: &'static str = "config.json";

fn main() {
    let opts: Opts = Opts::parse();
    let config = match Config::read_config() {
        Ok(v) => v,
        Err(err) => return println!("Error: {}", err.to_string()),
    };
    let maker = match Maker::new(config) {
        Ok(v) => v,
        Err(err) => return println!("Error: {}", err.to_string()),
    };
    match maker.check_current_directory(opts.force) {
        Ok(v) => v,
        Err(err) => return println!("Error: {}", err.to_string()),
    };

    maker.make().unwrap();
}
