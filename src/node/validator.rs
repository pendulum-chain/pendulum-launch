use super::Node;
use crate::Task;

#[derive(Debug)]
pub struct Validator<'a>(Node<'a>);

impl<'a> Validator<'a> {
    #[inline]
    pub fn new(node: Node<'a>) -> Self {
        Self(node)
    }

    pub fn create_task(&self) -> Task {
        let mut command = self.0.create_command();
        command.arg("--validator");

        Task::new(command)
    }
}
