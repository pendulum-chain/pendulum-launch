// COLLATOR
// ./target/release/parachain-collator
// --name pendulum_collator_1
// --collator
// --force-authoring
// --chain ./specs/rococo-local-parachain-raw.json
// --port 40333
// --ws-port 8844
// --
// --execution wasm
// --chain ./polkadot-launch/specs/rococo-custom-2-raw.json
// --port 30343
// --ws-port 9988

use super::{Command, Node, Run};
use std::{io, path::PathBuf, process};

#[derive(Debug)]
pub struct CollatorRelay<'a> {
    chain: &'a PathBuf,
    port: u16,
    ws_port: u16,
    rpc_port: Option<u16>,
}

impl<'a> CollatorRelay<'a> {
    pub fn new(chain: &'a PathBuf, port: u16, ws_port: u16, rpc_port: Option<u16>) -> Self {
        Self {
            chain,
            port,
            ws_port,
            rpc_port,
        }
    }
}

#[derive(Debug)]
pub struct Collator<'a> {
    inner: Node<'a>,
    relay: CollatorRelay<'a>,
}

impl<'a> Collator<'a> {
    pub fn new(inner: Node<'a>, relay: CollatorRelay<'a>) -> Self {
        Self { inner, relay }
    }
}

impl<'a> Command for Collator<'a> {
    fn as_command(&self) -> process::Command {
        let mut command = self.inner.as_command();
        command.arg("--collator");
        command.arg("--");
        command.arg("--execution");
        command.arg("wasm");
        command.arg("--chain");
        command.arg(self.relay.chain.to_str().unwrap());
        command.arg("--port");
        command.arg(self.relay.port.to_string());
        command.arg("--ws-port");
        command.arg(self.relay.ws_port.to_string());
        if let Some(rpc_port) = self.relay.rpc_port {
            command.arg("--rpc-port");
            command.arg(rpc_port.to_string());
        };

        command
    }
}

impl<'a> Run<process::Child> for Collator<'a> {
    fn run(&self) -> io::Result<process::Child> {
        Ok(self.as_command().spawn()?)
    }
}
