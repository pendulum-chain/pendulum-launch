use super::Node;
use crate::Task;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Validator(Node);

impl Validator {
    #[inline]
    pub fn new(node: Node) -> Self {
        Self(node)
    }

    pub fn create_task(&self) -> Task {
        let mut command = self.0.create_command();
        command.arg("--validator");

        Task::new(command)
    }
}
