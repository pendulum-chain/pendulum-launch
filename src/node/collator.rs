use super::Node;
use crate::{
    error::{Error, Result},
    PathBuffer, Task,
};
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

    pub fn create_task(&mut self, log_dir: &Option<PathBuffer>) -> Result<Task> {
        let args = self.get_args()?;
        Ok(Task::new(self.inner.create_command(args, log_dir)?))
    }

    #[inline]
    pub fn as_command(&mut self) -> Result<process::Command> {
        let args = self.get_args()?;
        self.inner.create_command(args, &None)
    }

    fn get_args(&mut self) -> Result<Vec<String>> {
        let chain = match self.relay.chain.to_str() {
            Some(chain) => chain,
            None => return Err(Error::InvalidPath),
        };

        let mut args = vec![
            "--collator".to_owned(),
            "--".to_owned(),
            "--execution".to_owned(),
            "wasm".to_owned(),
            "--chain".to_owned(),
            chain.to_owned(),
            "--port".to_owned(),
            self.relay.port.to_string(),
            "--ws-port".to_owned(),
            self.relay.ws_port.to_string(),
        ];

        // Append validator args if there are any, replacing them with None
        //
        // This is nothing of concern, as the Nodes are upon task initialization
        if let Some(mut validator_args) = self.relay.args.take() {
            args.append(&mut validator_args);
        };

        if let Some(rpc_port) = self.relay.rpc_port {
            args.push("--rpc-port".to_owned());
            args.push(rpc_port.to_string());
        };

        Ok(args)
    }
}
