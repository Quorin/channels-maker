use std::env::current_exe;
use std::fs::{self, create_dir, remove_dir, remove_file, DirEntry};
use std::path::PathBuf;

use crate::config::Config;
use snafu::{ResultExt, Snafu};

#[derive(Debug, Snafu)]
pub enum MakerError {
    #[snafu(display("cannot read current directory: {}", source))]
    Read { source: std::io::Error },
    #[snafu(display("cannot remove file {:?}: {}", path, source))]
    RemoveFile {
        source: std::io::Error,
        path: PathBuf,
    },
    #[snafu(display("cannot remove directory {:?}: {}", path, source))]
    RemoveDirectory {
        source: std::io::Error,
        path: PathBuf,
    },
    #[snafu(display(
        "you have to clean directory first or use flag --force to delete automatically"
    ))]
    NotEmpty,
    #[snafu(display("cannot make directory {:?}: {}", path, source))]
    CreateDirectory {
        source: std::io::Error,
        path: PathBuf,
    },
    #[snafu(display("cannot create file {:?}: {}", path, source))]
    CreateFile {
        source: std::io::Error,
        path: PathBuf,
    },
}

type MakerResult<T, E = MakerError> = std::result::Result<T, E>;

fn get_current_file_name() -> String {
    let path = PathBuf::from(current_exe().unwrap());
    let file_name = path.file_name().unwrap().to_str().unwrap();
    format!("./{}", file_name)
}

lazy_static! {
    static ref ALLOWED_DIRECTORIES: Vec<PathBuf> = vec![PathBuf::from("./share")];
    static ref ALLOWED_FILES: Vec<PathBuf> = vec![
        PathBuf::from(format!("./{}", crate::CONFIG_FILE)),
        PathBuf::from(get_current_file_name())
    ];
}

#[derive(Debug)]
pub struct Maker {
    config: Config,
    dirs: Vec<DirEntry>,
    files: Vec<DirEntry>,
}

impl Maker {
    pub fn new(config: Config) -> MakerResult<Self> {
        let dirs = Self::get_entries()?
            .into_iter()
            .filter(|d| d.path().is_dir())
            .collect::<Vec<_>>();
        let files = Self::get_entries()?
            .into_iter()
            .filter(|d| d.path().is_file())
            .collect::<Vec<_>>();

        Ok(Self {
            config,
            dirs,
            files,
        })
    }

    fn get_entries() -> MakerResult<Vec<DirEntry>> {
        Ok(std::fs::read_dir(".")
            .context(Read)?
            .filter_map(|d| d.ok())
            .collect::<Vec<_>>())
    }

    pub fn check_current_directory(&self, force: bool) -> MakerResult<()> {
        let mut not_allowed: Vec<&DirEntry> = vec![];

        self.dirs.iter().for_each(|d| {
            if !ALLOWED_DIRECTORIES.contains(&d.path()) {
                not_allowed.push(d);
            }
        });

        self.files.iter().for_each(|d| {
            if !ALLOWED_FILES.contains(&d.path()) {
                not_allowed.push(d);
            }
        });

        if !not_allowed.is_empty() {
            if !force {
                return Err(MakerError::NotEmpty);
            }

            for e in not_allowed.iter() {
                if e.path().is_dir() {
                    remove_dir(e.path()).context(RemoveDirectory { path: e.path() })?
                } else {
                    remove_file(e.path()).context(RemoveFile { path: e.path() })?
                }
            }
        }

        Ok(())
    }

    pub fn make(&self) -> MakerResult<()> {
        // auth
        create_dir("auth").context(CreateDirectory { path: "auth" })?;

        // channels
        for x in &self.config.channels.settings {
            create_dir(format!("./{}", x.channel_dir_name())).context(CreateDirectory {
                path: format!("./{}", x.channel_dir_name()),
            })?;

            let maps = x.get_map_ids(&self.config.channels);

            for part_id in 1..=maps.len() {
                create_dir(format!("./{}/part{}", x.channel_dir_name(), part_id)).context(
                    CreateDirectory {
                        path: format!("./{}/part{}", x.channel_dir_name(), part_id),
                    },
                )?;

                create_dir(format!("./{}/part{}/log", x.channel_dir_name(), part_id)).context(
                    CreateDirectory {
                        path: format!("./{}/part{}/log", x.channel_dir_name(), part_id),
                    },
                )?;

                create_dir(format!("./{}/part{}/mark", x.channel_dir_name(), part_id)).context(
                    CreateDirectory {
                        path: format!("./{}/part{}/mark", x.channel_dir_name(), part_id),
                    },
                )?;

                fs::write(
                    format!("./{}/part{}/CONFIG", x.channel_dir_name(), part_id),
                    format!(
                        "CHANNEL: {}
HOSTNAME: part{}
PORT: {}
P2P_PORT: {}
DB_ADDR: {}
DB_PORT: {}
PLAYER_SQL: {} {} {} {} {} {}
COMMON_SQL: {} {} {} {} {} {}
LOG_SQL: {} {} {} {} {} {}
TABLE_POSTFIX: {}
MAP_ALLOW: {}
PASSES_PER_SEC: {}
SAVE_EVENT_SECOND_CYCLE: {}
PING_EVENT_SECOND_CYCLE: {}
VIEW_RANGE: {}
CHECK_MULTIHACK: {}
LOCALE_SERVICE: {}
ADMINPAGE_PASSWORD: {}
adminpage_ip: {}
adminpage_ip1: {}
adminpage_ip2: {}
adminpage_ip3: {}
SPEEDHACK_LIMIT_COUNT: {}
SPEEDHACK_LIMIT_BONUS: {}
PK_PROTECT_LEVEL: {}
MALL_URL: {}
TRAFFIC_PROFILE: {}
TEST_SERVER: {}
MAX_LEVEL: {}
g_bDisableItemBonusChangeTime: {}
",
                        x.channel_id,
                        part_id,
                        x.port,
                        x.p2p_port,
                        self.config.common.db_ip,
                        self.config.common.db_port,
                        // player_sql
                        self.config.databases.player.ip,
                        self.config.databases.player.user,
                        self.config.databases.player.password,
                        self.config.databases.player.database,
                        self.config.databases.player.port,
                        self.config.databases.player.sock,
                        // common_sql
                        self.config.databases.common.ip,
                        self.config.databases.common.user,
                        self.config.databases.common.password,
                        self.config.databases.common.database,
                        self.config.databases.common.port,
                        self.config.databases.common.sock,
                        // log_sql
                        self.config.databases.log.ip,
                        self.config.databases.log.user,
                        self.config.databases.log.password,
                        self.config.databases.log.database,
                        self.config.databases.log.port,
                        self.config.databases.log.sock,
                        //
                        self.config.common.table_postfix,
                        maps[part_id - 1]
                            .iter()
                            .map(|m| format!(" {}", m))
                            .collect::<String>(),
                        self.config.common.passes_per_sec,
                        self.config.common.save_event_second_cycle,
                        self.config.common.ping_event_second_cycle,
                        self.config.common.view_range,
                        0,
                        self.config.common.locale_service,
                        // adminpage
                        self.config.adminpage_ips.password,
                        self.config.adminpage_ips.adminpage_ip,
                        self.config.adminpage_ips.adminpage_ip1,
                        self.config.adminpage_ips.adminpage_ip2,
                        self.config.adminpage_ips.adminpage_ip3,
                        //
                        self.config.common.speedhack_limit_count,
                        self.config.common.speedhack_limit_bonus,
                        self.config.common.pk_protect_level,
                        self.config.common.mall_url,
                        self.config.common.traffic_profile,
                        self.config.common.test_server,
                        self.config.common.max_level,
                        self.config.common.disable_item_bonus_change_time,
                    ),
                )
                .context(CreateFile {
                    path: format!("./{}/part{}/CONFIG", x.channel_dir_name(), part_id),
                })?
            }
        }

        // auth channels
        for x in 1..=self.config.auth.ports.len() {
            create_dir(format!("./auth/{}", x)).context(CreateDirectory {
                path: format!("./auth/{}", x),
            })?;

            create_dir(format!("./auth/{}/log", x)).context(CreateDirectory {
                path: format!("./auth/{}/log", x),
            })?;

            fs::write(
                format!("./auth/{}/CONFIG", x),
                format!(
                    "CHANNEL: {}
HOSTNAME: auth{}
PORT: {}
P2P_PORT: {}
DB_ADDR: {}
DB_PORT: {}
PLAYER_SQL: {} {} {} {} {} {}
COMMON_SQL: {} {} {} {} {} {}
LOG_SQL: {} {} {} {} {} {}
TABLE_POSTFIX: {}
PASSES_PER_SEC: {}
PING_EVENT_SECOND_CYCLE: {}
ADMINPAGE_PASSWORD: {}
adminpage_ip: {}
adminpage_ip1: {}
adminpage_ip2: {}
adminpage_ip3: {}
AUTH_SERVER: {}
TRAFFIC_PROFILE: {}
",
                    x,
                    x,
                    self.config.auth.ports[(x - 1)].port,
                    self.config.auth.ports[(x - 1)].p2p_port,
                    self.config.common.db_ip,
                    self.config.common.db_port,
                    // player_sql
                    self.config.databases.account.ip,
                    self.config.databases.account.user,
                    self.config.databases.account.password,
                    self.config.databases.account.database,
                    self.config.databases.account.port,
                    self.config.databases.account.sock,
                    // common_sql
                    self.config.databases.common.ip,
                    self.config.databases.common.user,
                    self.config.databases.common.password,
                    self.config.databases.common.database,
                    self.config.databases.common.port,
                    self.config.databases.common.sock,
                    // log_sql
                    self.config.databases.log.ip,
                    self.config.databases.log.user,
                    self.config.databases.log.password,
                    self.config.databases.log.database,
                    self.config.databases.log.port,
                    self.config.databases.log.sock,
                    //
                    self.config.common.table_postfix,
                    self.config.common.passes_per_sec,
                    self.config.common.ping_event_second_cycle,
                    // adminpage
                    self.config.adminpage_ips.password,
                    self.config.adminpage_ips.adminpage_ip,
                    self.config.adminpage_ips.adminpage_ip1,
                    self.config.adminpage_ips.adminpage_ip2,
                    self.config.adminpage_ips.adminpage_ip3,
                    //
                    self.config.auth.auth_server,
                    self.config.auth.traffic_profile
                ),
            )
            .context(CreateFile {
                path: format!("./auth/{}/CONFIG", x),
            })?
        }

        Ok(())
    }
}
