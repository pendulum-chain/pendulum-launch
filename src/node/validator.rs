use super::{AsCommand, Node};
use crate::util::path_to_str;
use crate::{error::Result, PathBuffer, Task};
use serde::{Deserialize, Serialize};
use std::process;
use std::rc::Rc;

#[derive(Debug, Deserialize, Serialize)]
pub struct Validator(Node);

impl Validator {
    #[inline]
    pub fn new(node: Node) -> Self {
        Self(node)
    }

    #[inline]
    pub fn create_task(&self, log_dir: Option<PathBuffer>) -> Result<Task> {
        Ok(Task::new(self.0.as_command_internal(
            vec!["--validator".to_owned()],
            log_dir,
        )?))
    }
}

impl AsRef<Node> for Validator {
    fn as_ref(&self) -> &Node {
        &self.0
    }
}

impl AsCommand for Validator {
    fn as_command_internal(&self, log_dir: Option<PathBuffer>) -> Result<process::Command> {
        self.as_ref()
            .create_command(self.0.args.to_owned(), &log_dir)
    }

    fn as_command_external(&self) -> Result<String> {
        let bin = self.as_ref().bin.as_os_str();
        // let args = self.as_ref().args
        Ok("".to_owned())
    }
}
