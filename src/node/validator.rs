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
        Ok(Task::new(self.as_command_internal()?))
    }
}

impl AsRef<Node> for Validator {
    fn as_ref(&self) -> &Node {
        &self.0
    }
}

impl AsCommand for Validator {
    fn as_command_internal(&self) -> Result<process::Command> {
        self.0.as_command_internal()
    }

    fn as_command_external(&self) -> Result<String> {
        self.0.as_command_external()
    }
}
