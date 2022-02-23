mod collator;
mod validator;

pub use collator::{Collator, CollatorRelay};
pub use validator::Validator;

use crate::{error::Result, launcher::LOG_DIR, util, PathBuffer};
use serde::{Deserialize, Serialize};
use std::{
    fs::File,
    process::{self, Stdio},
    sync::Arc,
};

pub trait AsCommand {
    fn as_command_internal(&self) -> Result<process::Command>;
    fn as_command_external(&self) -> Result<String>;
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Node {
    name: String,
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
        let name = match name {
            Some(name) => name.to_owned(),
            None => Self::get_name(bin, ws_port),
        };

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
        log_dir: Option<PathBuffer>,
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
            .arg("--name")
            .arg(&self.name)
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

        Ok(command)
    }

    fn get_args(&self) -> Result<Vec<String>> {
        let mut args = self.args.to_owned();
        args.append(
            vec![
                "--name".to_owned(),
                self.name.to_owned(),
                "--chain".to_owned(),
                util::path_to_str(self.chain.as_ref())?,
                "--port".to_owned(),
                self.port.to_string(),
                "--ws-port".to_owned(),
                self.ws_port.to_string(),
            ]
            .as_mut(),
        );

        if let Some(rpc_port) = self.rpc_port {
            args.push("--rpc-port".to_owned());
            args.push(rpc_port.to_string());
        };

        Ok(args)
    }

    pub fn get_log_name(&self) -> Result<String> {
        Ok(format!("{}.log", self.name))
    }

    fn get_name(bin: &str, ws_port: u16) -> String {
        format!("{}-{}", bin, ws_port)
    }
}

impl AsCommand for Node {
    fn as_command_internal(&self) -> Result<process::Command> {
        let log = match &*Arc::clone(&LOG_DIR).read()? {
            Some(path) => {
                let path = path.join(self.get_log_name()?);
                let file = File::create(path.as_ref())?;
                Stdio::from(file)
            }
            None => Stdio::null(),
        };

        let mut command = process::Command::new(self.bin.as_ref());
        command.stdout(log).args(self.get_args()?);

        Ok(command)
    }

    fn as_command_external(&self) -> Result<String> {
        let mut command = vec![util::path_to_str(self.bin.as_ref())?];
        command.append(self.get_args()?.as_mut());

        Ok(command.join(" "))
    }
}
