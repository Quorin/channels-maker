use std::env::current_exe;
use std::fs::{remove_dir, remove_file, DirEntry};
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
}
