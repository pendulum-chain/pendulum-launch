use super::Node;
use crate::error::Result;
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

    pub fn create_task(&self, quiet: bool, log_dir: &Option<PathBuffer>) -> Result<Task> {
        let mut command = self.inner.create_command(quiet);
        command
            .arg("--collator")
            .arg("--")
            .arg("--execution")
            .arg("wasm")
            .arg("--chain")
            .arg(self.relay.chain.as_os_str())
            .arg("--port")
            .arg(self.relay.port.to_string())
            .arg("--ws-port")
            .arg(self.relay.ws_port.to_string());

        if let Some(rpc_port) = self.relay.rpc_port {
            command.arg("--rpc-port");
            command.arg(rpc_port.to_string());
        };

        let log_file = match log_dir {
            Some(dir) => Some(dir.join(self.inner.get_log_name()?)),
            None => None,
        };

        Ok(Task::new(command, log_file))
    }
}
