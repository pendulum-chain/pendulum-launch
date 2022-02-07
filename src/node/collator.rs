use super::Node;
use crate::error::{Error, Result};
use crate::util::path_to_str;
use crate::{PathBuffer, Task};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct CollatorRelay {
    chain: PathBuffer,
    port: u16,
    ws_port: u16,
    rpc_port: Option<u16>,
}

impl CollatorRelay {
    pub fn new(chain: &str, port: u16, ws_port: u16, rpc_port: Option<u16>) -> Self {
        let chain = PathBuffer::from(chain);
        Self {
            chain,
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

    pub fn create_task(&self, log_dir: &Option<PathBuffer>) -> Result<Task> {
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

        if let Some(rpc_port) = self.relay.rpc_port {
            args.push("--rpc-port".to_owned());
            args.push(rpc_port.to_string());
        };

        Ok(Task::new(self.inner.create_command(args, log_dir)?))
    }
}
