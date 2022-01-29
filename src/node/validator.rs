// VALIDATOR
// ./bin/polkadot
// --bob
// --validator
// --chain ./specs/rococo-custom-2-raw.json
// --port 30334
// --ws-port 995

use super::{Command, Node, Run};
use std::{io, process};

#[derive(Debug)]
pub struct Validator<'a>(Node<'a>);

impl<'a> Validator<'a> {
    pub fn new(node: Node<'a>) -> Self {
        Self(node)
    }
}

impl<'a> Command for Validator<'a> {
    fn as_command(&self) -> process::Command {
        let mut command = self.0.as_command();
        command.arg("--validator");

        command
    }
}

impl<'a> Run<process::Child> for Validator<'a> {
    fn run(&self) -> io::Result<process::Child> {
        Ok(self.as_command().spawn()?)
    }
}
