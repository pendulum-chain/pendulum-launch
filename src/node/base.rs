use crate::{error::Result, launcher::LOG_DIR, util, PathBuffer};
use serde::{Deserialize, Serialize};
use std::{
    fs::File,
    process::{self, Stdio},
    sync::Arc,
};

use super::{AsCommand, Node};

#[derive(Debug, Deserialize, Serialize)]
pub struct BaseNode {
    name: String,
    bin: PathBuffer,
    chain: PathBuffer,
    dockerfile: Option<PathBuffer>,
    args: Vec<String>,
    port: u16,
    ws_port: u16,
    rpc_port: Option<u16>,
}

impl BaseNode {
    pub fn new(
        name: String,
        bin: PathBuffer,
        chain: PathBuffer,
        dockerfile: Option<PathBuffer>,
        args: Vec<String>,
        port: u16,
        ws_port: u16,
        rpc_port: Option<u16>,
    ) -> Self {
        Self {
            name,
            bin,
            chain,
            dockerfile,
            args,
            port,
            ws_port,
            rpc_port,
        }
    }

    #[inline]
    pub fn chain(&self) -> &PathBuffer {
        &self.chain
    }

    #[inline]
    pub fn get_log_name(&self) -> Result<String> {
        Ok(format!("{}.log", self.name))
    }
}

impl Node for BaseNode {
    #[inline]
    fn name(&self) -> &str {
        &self.name
    }

    fn args(&self) -> Result<Vec<String>> {
        let mut args = self.args.to_owned();

        args.append(
            vec![
                "--name".to_owned(),
                self.name.to_owned(),
                "--chain".to_owned(),
                util::path_to_string(self.chain.as_ref())?,
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

    #[inline]
    fn ports(&self) -> Vec<Option<u16>> {
        vec![self.port.into(), self.ws_port.into(), self.rpc_port]
    }

    #[inline]
    fn specs(&self) -> Result<Vec<String>> {
        Ok(vec![self.chain.to_string()?])
    }

    fn docker_file(&self) -> Result<String> {
        match &self.dockerfile {
            Some(path) => util::path_to_string(path.as_ref()),
            None => Ok("Dockerfile".to_owned()),
        }
    }
}

impl AsCommand for BaseNode {
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
        command.stdout(log).args(self.args()?);

        Ok(command)
    }

    fn as_command_external(&self, docker_volume: bool) -> Result<String> {
        let mut command = vec![util::path_to_string(self.bin.as_ref())?];
        command.append(self.args()?.as_mut());

        // Push container if `--enable-volume is enabled`
        if docker_volume {
            println!("mama mia pizzeria");
            command.push(format!("--mount {}:/specs", self.name));
        }

        Ok(command.join(" "))
    }
}
