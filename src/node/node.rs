use crate::PathBuffer;
use serde::Deserialize;
use std::process;

#[derive(Debug, Deserialize)]
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
        let name = match name {
            Some(name) => Some(name.to_owned()),
            None => None,
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

    pub fn create_command(&self) -> process::Command {
        let mut command = process::Command::new(self.bin.as_ref());
        command.args(self.args.clone());
        command.arg("--chain");
        command.arg(self.chain.as_os_str());
        command.arg("--port");
        command.arg(self.port.to_string());
        command.arg("--ws-port");
        command.arg(self.ws_port.to_string());
        if let Some(rpc_port) = self.rpc_port {
            command.arg(format!("--rpc-port {rpc_port}"));
        };

        command
    }
}
