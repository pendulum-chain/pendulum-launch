use std::{path::PathBuf, process};

mod collator;
mod command;
mod validator;

pub use collator::{Collator, CollatorRelay};
pub(super) use command::{Command, Run};
pub use validator::Validator;

#[derive(Debug)]
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
}

impl<'a> Command for Node<'a> {
    fn as_command(&self) -> process::Command {
        let mut command = process::Command::new(self.bin);
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
