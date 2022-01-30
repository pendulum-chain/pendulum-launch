use super::Node;
use crate::{PathBuffer, Task};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
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

#[derive(Debug, Deserialize)]
pub struct Collator {
    inner: Node,
    relay: CollatorRelay,
}

impl Collator {
    #[inline]
    pub fn new(inner: Node, relay: CollatorRelay) -> Self {
        Self { inner, relay }
    }

    pub fn create_task(&self) -> Task {
        let mut command = self.inner.create_command();
        command.arg("--collator");
        command.arg("--");
        command.arg("--execution");
        command.arg("wasm");
        command.arg("--chain");
        command.arg(self.relay.chain.as_os_str());
        command.arg("--port");
        command.arg(self.relay.port.to_string());
        command.arg("--ws-port");
        command.arg(self.relay.ws_port.to_string());
        if let Some(rpc_port) = self.relay.rpc_port {
            command.arg("--rpc-port");
            command.arg(rpc_port.to_string());
        };

        Task::new(command)
    }
}
