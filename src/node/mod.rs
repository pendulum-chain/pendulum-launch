mod collator;
mod validator;

pub use collator::{Collator, CollatorRelay};
pub use validator::Validator;

use crate::{
    error::{Error, Result},
    util, PathBuffer,
};
use serde::{Deserialize, Serialize};
use std::{
    fs::File,
    process::{self, Stdio},
};

pub trait AsCommand {
    fn as_command_internal(&self, log_dir: &Option<PathBuffer>) -> Result<process::Command>;
    fn as_command_external(&self) -> Result<String>;
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Node {
    name: Option<String>,
    bin: PathBuffer,
    chain: PathBuffer,
    args: Vec<String>,
    port: u16,
    ws_port: u16,
    rpc_port: Option<u16>,
}

impl Node {
    pub fn new(
        name: Option<&str>,
        bin: &str,
        chain: &str,
        args: Vec<&str>,
        port: u16,
        ws_port: u16,
        rpc_port: Option<u16>,
    ) -> Self {
        let name = name.map(|name| name.to_string());
        let bin = PathBuffer::from(bin);
        let chain = PathBuffer::from(chain);
        let args = args.into_iter().map(|arg| arg.to_owned()).collect();

        Self {
            name,
            bin,
            args,
            chain,
            port,
            ws_port,
            rpc_port,
        }
    }

    pub fn create_command(
        &self,
        args: Vec<String>,
        log_dir: &Option<PathBuffer>,
    ) -> Result<process::Command> {
        let log = match log_dir {
            Some(path) => {
                let path = path.join(self.get_log_name()?);
                let file = File::create(path.as_ref())?;
                Stdio::from(file)
            }
            None => Stdio::null(),
        };

        let mut command = process::Command::new(self.bin.as_ref());
        command
            .stdout(log)
            .args(self.args.to_owned())
            .arg("--chain")
            .arg(self.chain.as_os_str())
            .arg("--port")
            .arg(self.port.to_string())
            .arg("--ws-port")
            .arg(&self.ws_port.to_string())
            .args(args);

        if let Some(rpc_port) = self.rpc_port {
            command.arg("--rpc-port");
            command.arg(rpc_port.to_string());
        };

        // Use provided name or generate one if none is provided
        command.arg("--name");
        match &self.name {
            Some(name) => command.arg(name),
            None => command.arg(self.get_log_name()?),
        };

        Ok(command)
    }

    pub fn get_log_name(&self) -> Result<String> {
        let bin_path = util::path_to_str(self.bin.as_ref())?;
        let bin_name = match bin_path.split('/').last() {
            Some(bin) => bin,
            None => return Err(Error::InvalidPath),
        };
        let ws_port = self.ws_port;

        Ok(format!("{}-{}.log", bin_name, ws_port))
    }
}

impl AsCommand for Node {
    fn as_command_internal(&self, log_dir: &Option<PathBuffer>) -> Result<process::Command> {
        let log = match log_dir {
            Some(path) => {
                let path = path.join(self.get_log_name()?);
                let file = File::create(path.as_ref())?;
                Stdio::from(file)
            }
            None => Stdio::null(),
        };

        let mut command = process::Command::new(self.bin.as_ref());
        command
            .stdout(log)
            .args(self.args.to_owned())
            .arg("--chain")
            .arg(self.chain.as_os_str())
            .arg("--port")
            .arg(self.port.to_string())
            .arg("--ws-port")
            .arg(&self.ws_port.to_string());

        if let Some(rpc_port) = self.rpc_port {
            command.arg("--rpc-port");
            command.arg(rpc_port.to_string());
        };

        // Use provided name or generate one if none is provided
        command.arg("--name");
        match &self.name {
            Some(name) => command.arg(name),
            None => command.arg(self.get_log_name()?),
        };

        Ok(command)
    }

    fn as_command_external(&self) -> Result<String> {
        Ok("".to_owned())
    }
}
