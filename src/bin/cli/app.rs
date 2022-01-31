use crate::{Error, Opt, Result};
use lib_pendulum_launch::{
    node::{Collator, CollatorRelay, Node, Validator},
    Config, Launcher,
};
use std::{
    fs::{self, DirEntry},
    io,
    path::PathBuf,
};

pub enum Command {
    Launch,
    ExportGenesis,
    ExportSpecs,
}

pub struct App {
    launcher: Option<Launcher>,
    collator_bin: Option<PathBuf>,
    collator_spec: Option<PathBuf>,
}

impl App {
    pub fn new(
        launcher: Option<Launcher>,
        collator_bin: Option<PathBuf>,
        collator_spec: Option<PathBuf>,
    ) -> Self {
        Self {
            launcher,
            collator_bin,
            collator_spec,
        }
    }

    pub fn run(&mut self) -> Result<()> {
        match &mut self.launcher {
            Some(launcher) => match launcher.run() {
                Ok(()) => Ok(()),
                Err(err) => Err(Error::Lib(err)),
            },
            None => Err(Error::CollatorBin),
        }
    }

    fn validate(&self, command: Command) -> Result<()> {
        match command {
            Command::Launch => match self.launcher {
                Some(_) => Ok(()),
                None => Err(Error::LaunchCommand),
            },
            Command::ExportGenesis | Command::ExportSpecs => {
                match self.collator_bin.is_some() && self.collator_spec.is_some() {
                    true => Ok(()),
                    false => Err(Error::GenCommand),
                }
            }
        }
    }
}

impl TryFrom<Opt> for App {
    type Error = Error;

    fn try_from(opt: Opt) -> Result<Self> {
        let config = match opt.config {
            Some(config) => Some(config),
            None => search_default_config()?,
        };

        let launcher = match config {
            Some(path) => {
                let config = deserialize_config(path)?;
                Some(Launcher::from(config))
            }
            None => None,
        };

        Ok(Self::new(launcher, opt.collator_bin, opt.collator_spec))
    }
}

fn deserialize_config(path: PathBuf) -> Result<Config> {
    println!("here");
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

pub fn init() -> Launcher {
    let validator_bin = "./bin/polkadot";
    let validator_chain = "./specs/rococo-custom-2-raw.json";

    let collator_bin = "./bin/pendulum-collator";
    let collator_chain = "./specs/rococo-local-parachain-raw.json";

    let validator = {
        let name = Some("validator_node");
        let args = vec![];
        let port = 30343;
        let ws_port = 9944;
        let rpc_port = None;

        let node = Node::new(
            name,
            validator_bin,
            validator_chain,
            args,
            port,
            ws_port,
            rpc_port,
        );

        Validator::new(node)
    };

    let collator = {
        let name = Some("collator_node");
        let args = vec!["--force-authoring"];
        let port = 30344;
        let ws_port = 8844;
        let rpc_port = None;

        let relay = CollatorRelay::new(validator_chain, 30345, 9955, None);
        let inner = Node::new(
            name,
            collator_bin,
            collator_chain,
            args,
            port,
            ws_port,
            rpc_port,
        );

        Collator::new(inner, relay)
    };

    let config = Config::new(
        Some("Pendulum"),
        Some("xiuxiu"),
        vec![validator],
        vec![collator],
    );

    // Generate toml config
    // println!("{}", toml::to_string_pretty(&config).unwrap());
    // std::fs::write(
    //     &std::path::Path::new("./config/pendulum-launch.toml"),
    //     toml::to_vec(&config).unwrap(),
    // )
    // .unwrap();

    // Generate json config
    // println!("{}", serde_json::to_string_pretty(&config).unwrap());
    // std::fs::write(
    //     &std::path::Path::new("./config/pendulum-launch.json"),
    //     serde_json::to_string_pretty(&config).unwrap().as_bytes(),
    // )
    // .unwrap();

    Launcher::new(config)
}
