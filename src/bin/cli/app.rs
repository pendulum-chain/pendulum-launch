use crate::{opt::Command, util::locate_project_root, Options};
use lib_pendulum_launch::error::{Error, Result};
use std::{
    fs::{self, DirEntry},
    io,
    path::PathBuf,
};
use structopt::StructOpt;

pub struct App(Options);

impl App {
    pub fn new(options: Options) -> Self {
        Self(options)
    }

    pub fn from_args() -> Self {
        Self::new(Options::from_args())
    }

    pub fn run(&mut self) -> Result<()> {
        match &self.0.cmd {
            Some(cmd) => match cmd {
                Command::ExportGenesis {
                    collator_bin,
                    collator_spec,
                    outdir,
                } => self.export_genesis(
                    collator_bin.to_owned(),
                    collator_spec.to_owned(),
                    outdir.to_owned(),
                )?,
                Command::GenerateSpecs { .. } => eprintln!("Unimplemented"),
            },
            None => self.launch()?,
        };

        Ok(())
    }

    fn launch(&mut self) -> Result<()> {
        let config = match &self.0.config {
            Some(config) => Some(config.to_owned()),
            None => search_default_config()?,
        };

        let launcher = match config {
            Some(path) => {
                let config = deserialize_config(path)?;
                Some(lib_pendulum_launch::Launcher::from(config))
            }
            None => None,
        };

        match launcher {
            Some(mut launcher) => launcher.run(),
            None => Err(Error::InvalidPath),
        }
    }

    /// Export genesis data to an `outdir` if provided or to the project root
    fn export_genesis(&self, bin: PathBuf, chain: PathBuf, outdir: Option<PathBuf>) -> Result<()> {
        // Attempts to parse a PathBuf from a &str
        let path_to_str = |path: PathBuf| match path.to_str() {
            Some(path) => Ok(path.to_owned()),
            None => Err(Error::InvalidPath),
        };

        let bin = path_to_str(bin)?;
        let chain = path_to_str(chain)?;
        let outdir = {
            // Use project root if no `outdir` is provided
            let path = outdir.unwrap_or(locate_project_root()?);
            path_to_str(path)?
        };

        lib_pendulum_launch::export_genesis(bin, chain, outdir)?;
        Ok(())
    }
}

fn deserialize_config(path: PathBuf) -> Result<lib_pendulum_launch::Config> {
    lib_pendulum_launch::Config::deserialize(path)
}

fn search_default_config() -> Result<Option<PathBuf>> {
    for entry in fs::read_dir(".")? {
        if let Some(path) = try_get_config_entry(entry)? {
            return Ok(Some(path));
        }
    }

    Ok(None)
}

fn try_get_config_entry(entry: io::Result<DirEntry>) -> Result<Option<PathBuf>> {
    let path = entry?.path();
    if path.is_file() {
        let path_name = path.as_os_str();
        if path_name == "launch.toml" || path_name == "launch.json" {
            return Ok(Some(path));
        }
    }

    Ok(None)
}
