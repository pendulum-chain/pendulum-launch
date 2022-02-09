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

    #[inline]
    pub fn create_task(&self, log_dir: &Option<PathBuffer>) -> Result<Task> {
        Ok(Task::new(
            self.0
                .create_command(vec!["--validator".to_owned()], log_dir)?,
        ))
    }
}
