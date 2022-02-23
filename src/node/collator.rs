use super::{AsCommand, Node};
use crate::{error::Result, util, PathBuffer, Task};
use serde::{Deserialize, Serialize};
use std::process;

#[derive(Debug, Deserialize, Serialize)]
pub struct CollatorRelay {
    chain: PathBuffer,
    args: Option<Vec<String>>,
    port: u16,
    ws_port: u16,
    rpc_port: Option<u16>,
}

impl CollatorRelay {
    pub fn new(
        chain: &str,
        args: Option<Vec<String>>,
        port: u16,
        ws_port: u16,
        rpc_port: Option<u16>,
    ) -> Self {
        let chain = PathBuffer::from(chain);
        Self {
            chain,
            args,
            port,
            ws_port,
            rpc_port,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Collator {
    inner: Node,
    relay: CollatorRelay,
}

impl Collator {
    #[inline]
    pub fn new(inner: Node, relay: CollatorRelay) -> Self {
        Self { inner, relay }
    }

    pub fn create_task(&self) -> Result<Task> {
        let mut command = self.inner.as_command_internal()?;
        command.args(self.get_args()?);

        Ok(Task::new(command))
    }

    #[inline]
    pub fn as_command(&mut self) -> Result<process::Command> {
        let args = self.get_args()?;
        self.inner.create_command(args, None)
    }

    #[inline]
    pub fn name(&self) -> &str {
        &self.inner.name
    }

    pub fn ports(&self) -> [Option<u16>; 6] {
        [
            self.inner.port.into(),
            self.inner.ws_port.into(),
            self.inner.rpc_port,
            self.relay.port.into(),
            self.relay.ws_port.into(),
            self.relay.rpc_port,
        ]
    }

    fn get_args(&self) -> Result<Vec<String>> {
        let mut args = vec![
            "--collator".to_owned(),
            "--".to_owned(),
            "--chain".to_owned(),
            util::path_to_str(self.relay.chain.as_ref())?,
            "--port".to_owned(),
            self.relay.port.to_string(),
            "--ws-port".to_owned(),
            self.relay.ws_port.to_string(),
        ];

        // Append validator args if there are any, replacing them with None
        //
        // This is nothing of concern, as the Nodes are upon task initialization
        if let Some(mut validator_args) = self.relay.args.clone() {
            args.append(&mut validator_args);
        };

        if let Some(rpc_port) = self.relay.rpc_port {
            args.push("--rpc-port".to_owned());
            args.push(rpc_port.to_string());
        };

        Ok(args)
    }
}

impl AsCommand for Collator {
    fn as_command_internal(&self) -> Result<process::Command> {
        let mut command = self.inner.as_command_internal()?;
        command.args(self.get_args()?);

        Ok(command)
    }

    fn as_command_external(&self) -> Result<String> {
        let mut command = self.inner.as_command_external()?;
        command.push(' ');
        command.push_str(&self.get_args()?.join(" "));

        Ok(command)
    }
}
