#![allow(dead_code)]

use std::error::Error;
use std::path::PathBuf;
use std::process::{self, Command};

pub struct Node<'a> {
    name: Option<&'a str>,
    bin: &'a PathBuf,
    chain: &'a PathBuf,
    args: Vec<&'a str>,
    port: u16,
    ws_port: u16,
    rpc_port: Option<u16>,
}

impl<'a> Node<'a> {
    pub fn new(
        name: Option<&'a str>,
        bin: &'a PathBuf,
        chain: &'a PathBuf,
        args: Vec<&'a str>,
        port: u16,
        ws_port: u16,
        rpc_port: Option<u16>,
    ) -> Self {
        Self {
            bin,
            name,
            args,
            chain,
            port,
            ws_port,
            rpc_port,
        }
    }

    pub fn as_command(&self) -> Command {
        let mut command = Command::new(self.bin);
        command.args(self.args.clone());
        command.arg("--chain");
        command.arg(self.chain.to_str().unwrap());
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

pub struct Validator<'a>(Node<'a>);

impl<'a> Validator<'a> {
    pub fn new(node: Node<'a>) -> Self {
        Self(node)
    }

    fn as_command(&self) -> Command {
        let mut command = self.0.as_command();
        command.arg("--validator");
        println!("Validator: {command:?}");

        command
    }

    pub fn run(&self) -> Result<process::Child, Box<dyn Error>> {
        Ok(self.as_command().spawn()?)
    }
}

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

pub struct Collator<'a> {
    inner: Node<'a>,
    relay: CollatorRelay<'a>,
}

impl<'a> Collator<'a> {
    pub fn new(inner: Node<'a>, relay: CollatorRelay<'a>) -> Self {
        Self { inner, relay }
    }

    pub fn as_command(&self) -> Command {
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

        println!("Collator: {command:?}");

        command
    }

    pub fn run(&self) -> Result<process::Child, Box<dyn Error>> {
        Ok(self.as_command().spawn()?)
    }
}

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

// RELAY
// ./bin/polkadot
// --bob
// --validator
// --chain ./specs/rococo-custom-2-raw.json
// --port 30334
// --ws-port 995
