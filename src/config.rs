use crate::node::{Collator, Validator};
use crate::Task;

#[derive(Debug)]
pub struct Config<'a> {
    pub validators: Vec<Validator<'a>>,
    pub collators: Vec<Collator<'a>>,
}

impl<'a> Config<'a> {
    pub fn new(validators: Vec<Validator<'a>>, collators: Vec<Collator<'a>>) -> Self {
        Self {
            validators,
            collators,
        }
    }

    pub fn generate_tasks(&self) -> Vec<Task> {
        let validator_tasks = self.validators.iter().map(|v| v.create_task());
        let collator_tasks = self.collators.iter().map(|c| c.create_task());
        validator_tasks.chain(collator_tasks).collect()
    }
}
