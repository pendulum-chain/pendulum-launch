use lib_pendulum_launch::{util, Config, Error, Result};
use std::{
    fs::{self, DirEntry},
    io,
    path::PathBuf,
};

/// Attempts to deserialize a config, searching for a default config if none is provided
pub fn deserialize_config(path: &Option<PathBuf>) -> Result<Config> {
    let path = {
        let path = match &path {
            Some(path) => Some(path.to_owned()),
            None => search_default_config()?,
        };

        match path {
            Some(path) => path,
            None => return Err(Error::NoConfig),
        }
    };

    Config::deserialize(path)
}

pub fn search_default_config() -> Result<Option<PathBuf>> {
    for entry in fs::read_dir(util::locate_project_root()?)? {
        if let Some(path) = try_get_config_entry(entry)? {
            return Ok(Some(path));
        }
    }

    Ok(None)
}

pub fn try_get_config_entry(entry: io::Result<DirEntry>) -> Result<Option<PathBuf>> {
    let path = entry?.path();
    match path.is_file() && util::path_to_string(&path)?.contains("launch.json") {
        true => Ok(Some(path)),
        false => Ok(None),
    }
}
