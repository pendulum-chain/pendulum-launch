mod collator;
mod validator;

pub use collator::{Collator, CollatorRelay};
pub use validator::Validator;

use crate::error::Error;
use crate::{error::Result, util, PathBuffer};
use serde::{Deserialize, Serialize};
use std::process::{self, Stdio};

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

    pub fn create_command(&self, quiet: bool) -> process::Command {
        let io_mode = if quiet { Stdio::null } else { Stdio::piped };
        let mut command = process::Command::new(self.bin.as_ref());
        command
            .stdout(io_mode())
            .stderr(io_mode())
            .args(self.args.to_owned())
            .arg("--chain")
            .arg(self.chain.as_os_str())
            .arg("--port")
            .arg(self.port.to_string())
            .arg("--ws-port")
            .arg(&self.ws_port.to_string());

        if let Some(rpc_port) = self.rpc_port {
            command.arg(format!("--rpc-port {}", rpc_port));
        };

        command
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
