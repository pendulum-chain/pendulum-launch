use std::path::PathBuf;
use std::process;
use std::rc::Rc;

#[derive(Debug)]
pub struct Node<'a> {
    name: Option<&'a str>,
    bin: Rc<PathBuf>,
    chain: Rc<PathBuf>,
    args: Vec<&'a str>,
    port: u16,
    ws_port: u16,
    rpc_port: Option<u16>,
}

impl<'a> Node<'a> {
    pub fn new(
        name: Option<&'a str>,
        bin: Rc<PathBuf>,
        chain: Rc<PathBuf>,
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
