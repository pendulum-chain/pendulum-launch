use super::{base::BaseNode, AsCommand, Node};
use crate::{config::ValidatorConfig, error::Result, Task};
use std::process;

#[derive(Debug)]
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

impl From<ValidatorConfig> for Validator {
    fn from(validator_config: ValidatorConfig) -> Self {
        validator_config.into()
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

    #[inline]
    fn specs(&self) -> Result<Vec<String>> {
        Ok(vec![self.0.chain().to_string()?])
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

    fn as_command_external(&self, docker_volume: bool) -> Result<String> {
        let mut command = self.as_ref().as_command_external(docker_volume)?;
        command.push(' ');
        command.push_str(&self.args()?.join(" "));

        Ok(command)
    }
}
