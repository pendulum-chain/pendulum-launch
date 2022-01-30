use crate::node::{Collator, Validator};
use crate::Task;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub name: Option<String>,
    pub author: Option<String>,
    pub validators: Vec<Validator>,
    pub collators: Vec<Collator>,
}

impl Config {
    pub fn new(
        name: Option<&str>,
        author: Option<&str>,
        validators: Vec<Validator>,
        collators: Vec<Collator>,
    ) -> Self {
        let name = name.map(|name| name.to_string());
        let author = author.map(|author| author.to_string());

        Self {
            name,
            author,
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
