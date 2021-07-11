use serde::{Deserialize, Serialize};
use std::fs::read_to_string;
use std::path::Path;

use snafu::{ResultExt, Snafu};

#[derive(Debug, Snafu)]
pub enum ConfigError {
    #[snafu(display("{} file not found", crate::CONFIG_FILE))]
    NotFound,
    #[snafu(display("cannot read config file: {}", source))]
    Read { source: std::io::Error },
    #[snafu(display("cannot parse config file: {}", source))]
    Parse { source: serde_json::Error },
}

pub type ConfigResult<T, E = ConfigError> = std::result::Result<T, E>;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    #[serde(rename = "server_name")]
    pub server_name: String,

    #[serde(rename = "auth")]
    pub auth: Auth,

    #[serde(rename = "channels")]
    pub channels: Channels,

    #[serde(rename = "common")]
    pub common: Common,

    #[serde(rename = "db")]
    pub db: Db,

    #[serde(rename = "adminpage_ips")]
    pub adminpage_ips: AdminpageIps,

    #[serde(rename = "databases")]
    pub databases: Databases,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AdminpageIps {
    #[serde(rename = "adminpage_ip")]
    pub adminpage_ip: String,

    #[serde(rename = "adminpage_ip1")]
    pub adminpage_ip1: String,

    #[serde(rename = "adminpage_ip2")]
    pub adminpage_ip2: String,

    #[serde(rename = "adminpage_ip3")]
    pub adminpage_ip3: String,

    #[serde(rename = "password")]
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Auth {
    #[serde(rename = "auth_server")]
    pub auth_server: String,

    #[serde(rename = "traffic_profile")]
    pub traffic_profile: i64,

    #[serde(rename = "ports")]
    pub ports: Vec<Port>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Port {
    #[serde(rename = "port")]
    pub port: i64,

    #[serde(rename = "p2p_port")]
    pub p2p_port: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Channels {
    #[serde(rename = "common_maps")]
    pub common_maps: Vec<Vec<i64>>,

    #[serde(rename = "settings")]
    pub settings: Vec<Setting>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Setting {
    #[serde(rename = "rename")]
    pub rename: Option<String>,

    #[serde(rename = "channel_id")]
    pub channel_id: i64,

    #[serde(rename = "port")]
    pub port: i64,

    #[serde(rename = "p2p_port")]
    pub p2p_port: i64,

    #[serde(rename = "override_maps")]
    pub override_maps: Option<Vec<Vec<i64>>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Common {
    #[serde(rename = "table_postfix")]
    pub table_postfix: String,

    #[serde(rename = "passes_per_sec")]
    pub passes_per_sec: i64,

    #[serde(rename = "db_ip")]
    pub db_ip: String,

    #[serde(rename = "db_port")]
    pub db_port: i64,

    #[serde(rename = "save_event_second_cycle")]
    pub save_event_second_cycle: i64,

    #[serde(rename = "ping_event_second_cycle")]
    pub ping_event_second_cycle: i64,

    #[serde(rename = "view_range")]
    pub view_range: i64,

    #[serde(rename = "locale_service")]
    pub locale_service: String,

    #[serde(rename = "speedhack_limit_count")]
    pub speedhack_limit_count: i64,

    #[serde(rename = "speedhack_limit_bonus")]
    pub speedhack_limit_bonus: i64,

    #[serde(rename = "pk_protect_level")]
    pub pk_protect_level: i64,

    #[serde(rename = "mall_url")]
    pub mall_url: String,

    #[serde(rename = "traffic_profile")]
    pub traffic_profile: i64,

    #[serde(rename = "test_server")]
    pub test_server: i64,

    #[serde(rename = "max_level")]
    pub max_level: i64,

    #[serde(rename = "disable_item_bonus_change_time")]
    pub disable_item_bonus_change_time: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Databases {
    #[serde(rename = "player")]
    pub player: Database,

    #[serde(rename = "common")]
    pub common: Database,

    #[serde(rename = "log")]
    pub log: Database,

    #[serde(rename = "account")]
    pub account: Database,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Database {
    #[serde(rename = "ip")]
    pub ip: String,

    #[serde(rename = "port")]
    pub port: String,

    #[serde(rename = "database")]
    pub database: String,

    #[serde(rename = "user")]
    pub user: String,

    #[serde(rename = "password")]
    pub password: String,

    #[serde(rename = "sock")]
    pub sock: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Db {
    #[serde(rename = "bind_port")]
    pub bind_port: i64,

    #[serde(rename = "db_sleep_msec")]
    pub db_sleep_msec: i64,

    #[serde(rename = "client_heart_fps")]
    pub client_heart_fps: i64,

    #[serde(rename = "hash_player_life_sec")]
    pub hash_player_life_sec: i64,

    #[serde(rename = "player_delete_level_limit")]
    pub player_delete_level_limit: i64,

    #[serde(rename = "player_id_start")]
    pub player_id_start: i64,

    #[serde(rename = "item_id_range")]
    pub item_id_range: ItemIdRange,

    #[serde(rename = "test_server")]
    pub test_server: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ItemIdRange {
    #[serde(rename = "start")]
    pub start: i64,

    #[serde(rename = "end")]
    pub end: i64,
}

impl Setting {
    pub fn channel_dir_name(&self) -> String {
        if let Some(rename_value) = self.rename.clone() {
            return rename_value;
        }

        return format!("channel{}", self.channel_id);
    }

    pub fn get_map_ids(&self, channels: &Channels) -> Vec<Vec<i64>> {
        if self.override_maps.is_none() {
            return channels.common_maps.clone();
        }

        self.override_maps.as_ref().unwrap().clone()
    }
}

impl Config {
    pub fn read_config() -> ConfigResult<Config> {
        let path = format!("./{}", crate::CONFIG_FILE);
        let file = Path::new(&path);
        if !file.exists() {
            return Err(ConfigError::NotFound);
        }

        let data = read_to_string(file).context(Read)?;
        Ok(serde_json::from_str::<Config>(&*data).context(Parse)?)
    }
}
