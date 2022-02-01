use crate::{opt::Command, Error, Options, Result};
use lib_pendulum_launch::{
    node::{Collator, CollatorRelay, Node, Validator},
    Config, Launcher,
};
use std::{
    fs::{self, DirEntry},
    io,
    path::PathBuf,
    process,
};
use structopt::StructOpt;

pub struct App(Options);
// {
//     launcher: Option<Launcher>,
// collator_bin: Option<PathBuf>,
// collator_spec: Option<PathBuf>,
// }

impl App {
    // pub fn new(
    //     launcher: Option<Launcher>,
    //     // collator_bin: Option<PathBuf>,
    //     // collator_spec: Option<PathBuf>,
    // ) -> Self {
    //     Self {
    //         launcher,
    //         // collator_bin,
    //         // collator_spec,
    //     }
    // }

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
                } => self.export_genesis(collator_bin, collator_spec, outdir)?,
                Command::GenerateSpecs { .. } => eprintln!("Unimplemented"),
            },
            None => self.launch()?,
        };

        Ok(())
    }

    // pub fn from_args() -> Result<Self> {
    //     Self::try_from(Options::from_args())
    // }

    fn launch(&mut self) -> Result<()> {
        let config = match &self.0.config {
            Some(config) => Some(config.to_owned()),
            None => search_default_config()?,
        };

        let launcher = match config {
            Some(path) => {
                let config = deserialize_config(path)?;
                Some(Launcher::from(config))
            }
            None => None,
        };

        match launcher {
            Some(mut launcher) => match launcher.run() {
                Ok(()) => Ok(()),
                Err(err) => Err(Error::Lib(err)),
            },
            None => Err(Error::InvalidBinary),
        }
    }

    fn export_genesis(
        &self,
        bin: &PathBuf,
        chain: &PathBuf,
        outdir: &Option<PathBuf>,
    ) -> Result<()> {
        let collator_bin = match bin.to_str() {
            Some(path) => path,
            None => return Err(Error::InvalidBinary),
        };

        let collator_spec = match chain.to_str() {
            Some(path) => path,
            None => return Err(Error::InvalidDirectory),
        };

        let genesis_outdir = match &outdir {
            Some(dir) => match dir.to_str() {
                Some(path) => path,
                None => return Err(Error::InvalidDirectory),
            },
            None => ".",
        };

        process::Command::new(collator_bin)
            .args([
                "export-genesis-wasm",
                "--chain",
                collator_spec,
                // ">",
                // format!("{genesis_outdir}/chain-wasm").as_str(),
            ])
            .status()?;

        process::Command::new(collator_bin)
            .args([
                "export-genesis-state",
                "--chain",
                collator_spec,
                // ">",
                // format!("{genesis_outdir}/chain-state").as_str(),
            ])
            .status()?;

        Ok(())
    }

    //     fn validate(&self, command: Command) -> Result<()> {
    //         match command {
    //             Command::Launch => match self.launcher {
    //                 Some(_) => Ok(()),
    //                 None => Err(Error::LaunchCommand),
    //             },
    //             Command::ExportGenesis | Command::ExportSpecs => {
    //                 match self.collator_bin.is_some() && self.collator_spec.is_some() {
    //                     true => Ok(()),
    //                     false => Err(Error::GenCommand),
    //                 }
    //             }
    //         }
    //     }
}

// impl TryFrom<Options> for App {
//     type Error = Error;

//     fn try_from(options: Options) -> Result<Self> {
//         let config = match options.config {
//             Some(config) => Some(config),
//             None => search_default_config()?,
//         };

//         let launcher = match config {
//             Some(path) => {
//                 let config = deserialize_config(path)?;
//                 Some(Launcher::from(config))
//             }
//             None => None,
//         };

//         // Ok(Self::new(launcher, options.collator_bin, options.collator_spec))
//         Ok(Self::new(launcher))
//     }
// }

fn deserialize_config(path: PathBuf) -> Result<Config> {
    match Config::deserialize(path) {
        Ok(config) => Ok(config),
        Err(err) => Err(Error::Lib(err)),
    }
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
