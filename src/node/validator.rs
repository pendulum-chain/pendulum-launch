use super::Node;
use crate::{error::Result, PathBuffer, Task};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Validator(Node);

impl Validator {
    #[inline]
    pub fn new(node: Node) -> Self {
        Self(node)
    }

    pub fn create_task(&self, quiet: bool, log_dir: &Option<PathBuffer>) -> Result<Task> {
        let mut command = self.0.create_command(quiet);
        command.arg("--validator");

        let log_file = match log_dir {
            Some(dir) => Some(dir.join(self.0.get_log_name()?)),
            None => None,
        };

        Ok(Task::new(command, log_file))
    }
}
