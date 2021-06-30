use crate::config::Config;

mod config;

fn main() {
    println!("{:?}", serde_json::from_str::<Config>(&std::fs::read_to_string("./config.json").unwrap()).unwrap())
}
