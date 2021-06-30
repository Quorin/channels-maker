use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    #[serde(rename = "server_name")]
    server_name: String,

    #[serde(rename = "auth")]
    auth: Auth,

    #[serde(rename = "channels")]
    channels: Channels,

    #[serde(rename = "common")]
    common: Common,

    #[serde(rename = "db")]
    db: Db,

    #[serde(rename = "adminpage_ips")]
    adminpage_ips: AdminpageIps,

    #[serde(rename = "databases")]
    databases: Databases,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AdminpageIps {
    #[serde(rename = "adminpage_ip")]
    adminpage_ip: String,

    #[serde(rename = "adminpage_ip1")]
    adminpage_ip1: String,

    #[serde(rename = "adminpage_ip2")]
    adminpage_ip2: String,

    #[serde(rename = "adminpage_ip3")]
    adminpage_ip3: String,

    #[serde(rename = "password")]
    password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Auth {
    #[serde(rename = "auth_server")]
    auth_server: String,

    #[serde(rename = "traffic_profile")]
    traffic_profile: i64,

    #[serde(rename = "ports")]
    ports: Vec<Port>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Port {
    #[serde(rename = "port")]
    port: i64,

    #[serde(rename = "p2p_port")]
    p2_p_port: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Channels {
    #[serde(rename = "common_maps")]
    common_maps: Vec<Vec<i64>>,

    #[serde(rename = "settings")]
    settings: Vec<Setting>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Setting {
    #[serde(rename = "channel_id")]
    channel_id: i64,

    #[serde(rename = "port")]
    port: i64,

    #[serde(rename = "p2p_port")]
    p2_p_port: i64,

    #[serde(rename = "override_maps")]
    override_maps: Option<Vec<Vec<i64>>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Common {
    #[serde(rename = "table_postfix")]
    table_postfix: String,

    #[serde(rename = "passes_per_sec")]
    passes_per_sec: i64,

    #[serde(rename = "db_ip")]
    db_ip: String,

    #[serde(rename = "db_port")]
    db_port: i64,

    #[serde(rename = "save_event_second_cycle")]
    save_event_second_cycle: i64,

    #[serde(rename = "ping_event_second_cycle")]
    ping_event_second_cycle: i64,

    #[serde(rename = "view_range")]
    view_range: i64,

    #[serde(rename = "locale_service")]
    locale_service: String,

    #[serde(rename = "speedhack_limit_count")]
    speedhack_limit_count: i64,

    #[serde(rename = "speedhack_limit_bonus")]
    speedhack_limit_bonus: i64,

    #[serde(rename = "pk_protect_level")]
    pk_protect_level: i64,

    #[serde(rename = "mall_url")]
    mall_url: String,

    #[serde(rename = "traffic_profile")]
    traffic_profile: i64,

    #[serde(rename = "test_server")]
    test_server: i64,

    #[serde(rename = "max_level")]
    max_level: i64,

    #[serde(rename = "disable_item_bonus_change_time")]
    disable_item_bonus_change_time: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Databases {
    #[serde(rename = "player")]
    player: Account,

    #[serde(rename = "common")]
    common: Account,

    #[serde(rename = "log")]
    log: Account,

    #[serde(rename = "account")]
    account: Account,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Account {
    #[serde(rename = "ip")]
    ip: String,

    #[serde(rename = "port")]
    port: String,

    #[serde(rename = "database")]
    database: String,

    #[serde(rename = "user")]
    user: String,

    #[serde(rename = "password")]
    password: String,

    #[serde(rename = "sock")]
    sock: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Db {
    #[serde(rename = "bind_port")]
    bind_port: i64,

    #[serde(rename = "db_sleep_msec")]
    db_sleep_msec: i64,

    #[serde(rename = "client_heart_fps")]
    client_heart_fps: i64,

    #[serde(rename = "hash_player_life_sec")]
    hash_player_life_sec: i64,

    #[serde(rename = "player_delete_level_limit")]
    player_delete_level_limit: i64,

    #[serde(rename = "player_id_start")]
    player_id_start: i64,

    #[serde(rename = "item_id_range")]
    item_id_range: ItemIdRange,

    #[serde(rename = "test_server")]
    test_server: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ItemIdRange {
    #[serde(rename = "start")]
    start: i64,

    #[serde(rename = "end")]
    end: i64,
}
