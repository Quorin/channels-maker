use std::env::current_exe;
use std::fs::DirEntry;
use std::path::PathBuf;

use crate::config::Config;
use crate::error::{MakerError, MakerResult};

lazy_static! {
    static ref ALLOWED_DIRECTORIES: Vec<PathBuf> = vec![PathBuf::from("./share")];
    static ref ALLOWED_FILES: Vec<PathBuf> = vec![PathBuf::from("./config.json"), current_exe().unwrap()];
}

#[derive(Debug)]
pub struct Maker {
    config: Config,
    dirs: Vec<DirEntry>,
    files: Vec<DirEntry>,
}

impl Maker {
    pub fn new(config: Config) -> MakerResult<Self> {
        let dirs = Self::get_entries()?.into_iter().filter(|d| d.path().is_dir()).collect::<Vec<_>>();
        let files = Self::get_entries()?.into_iter().filter(|d| d.path().is_file()).collect::<Vec<_>>();

        Ok(Self {
            config,
            dirs,
            files,
        })
    }

    fn get_entries() -> MakerResult<Vec<DirEntry>> {
        Ok(std::fs::read_dir(".")?.filter_map(|d| d.ok()).collect::<Vec<_>>())
    }

    pub fn check_current_directory(&self) -> MakerResult<()> {
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
            let file_names = not_allowed.iter().map(|e| e.file_name()).collect::<Vec<_>>();
            return Err(MakerError::DirectoryNotEmpty(file_names));
        }

        Ok(())
    }
}