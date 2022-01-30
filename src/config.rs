use crate::node::{Collator, Validator};
use crate::Task;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub validators: Vec<Validator>,
    pub collators: Vec<Collator>,
}

impl Config {
    pub fn new(validators: Vec<Validator>, collators: Vec<Collator>) -> Self {
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
