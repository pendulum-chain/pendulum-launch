use super::{AsCommand, Node};
use crate::{error::Result, Task};
use serde::{Deserialize, Serialize};
use std::process;

#[derive(Debug, Deserialize, Serialize)]
pub struct Validator(Node);

impl Validator {
    #[inline]
    pub fn new(node: Node) -> Self {
        Self(node)
    }

    #[inline]
    pub fn create_task(&self) -> Result<Task> {
        Ok(Task::new(self.0.as_command_internal()?))
    }
}

impl AsRef<Node> for Validator {
    fn as_ref(&self) -> &Node {
        &self.0
    }
}

impl AsCommand for Validator {
    fn as_command_internal(&self) -> Result<process::Command> {
        let mut cmd = self.0.as_command_internal()?;
        cmd.args(self.0.args.clone());

        Ok(cmd)
    }

    fn as_command_external(&self) -> Result<String> {
        // let bin = self.as_ref().bin.as_os_str();
        // let args = self.as_ref().args;
        Ok("".to_owned())
    }
}
