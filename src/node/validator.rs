use super::{AsCommand, BaseNode, Node};
use crate::{error::Result, Task};
use serde::{Deserialize, Serialize};
use std::process;

#[derive(Debug, Deserialize, Serialize)]
pub struct Validator(BaseNode);

impl Validator {
    #[inline]
    pub fn new(node: BaseNode) -> Self {
        Self(node)
    }

    #[inline]
    pub fn create_task(&self) -> Result<Task> {
        Ok(Task::new(self.as_command_internal()?))
    }
}

impl AsRef<BaseNode> for Validator {
    fn as_ref(&self) -> &BaseNode {
        &self.0
    }
}

impl Node for Validator {
    fn name(&self) -> &str {
        self.as_ref().name()
    }

    fn args(&self) -> Result<Vec<String>> {
        Ok(vec!["--validator".to_owned()])
    }

    fn ports(&self) -> Vec<Option<u16>> {
        self.as_ref().ports()
    }

    fn docker_file(&self) -> Result<String> {
        self.0.docker_file()
    }
}

impl AsCommand for Validator {
    fn as_command_internal(&self) -> Result<process::Command> {
        let mut command = self.as_ref().as_command_internal()?;
        command.args(self.args()?);

        Ok(command)
    }

    fn as_command_external(&self) -> Result<String> {
        let mut command = self.as_ref().as_command_external()?;
        command.push(' ');
        command.push_str(&self.args()?.join(" "));

        Ok(command)
    }
}
