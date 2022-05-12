use crate::{
    error::{Error, Result, SerdeError},
    node::{Collator, Node, Validator},
    Task,
};
use serde::{Deserialize, Serialize};
use std::{collections::HashSet, fs, path::PathBuf};

mod collator;
mod validator;

pub use collator::CollatorConfig;
pub use validator::ValidatorConfig;

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub name: Option<String>,
    pub author: Option<String>,
    pub validators: Vec<ValidatorConfig>,
    pub collators: Vec<CollatorConfig>,
}

impl Config {
    pub fn new(
        name: Option<&str>,
        author: Option<&str>,
        validators: Vec<ValidatorConfig>,
        collators: Vec<CollatorConfig>,
    ) -> Self {
        let name = name.map(|name| name.to_string());
        let author = author.map(|author| author.to_string());
        // let validators = validators.into_iter().map(|v| Validator::from(v)).collect();
        // let collators = collators.into_iter().map(|c| Collator::from(c)).collect();

        Self {
            name,
            author,
            validators,
            collators,
        }
    }

    pub fn deserialize(path: PathBuf) -> Result<Self> {
        let raw_config = &fs::read(path)?;
        match serde_json::from_slice(raw_config) {
            Ok(config) => Ok(config),
            Err(err) => Err(Error::Serde(SerdeError::Deserialize(err.to_string()))),
        }
    }

    pub fn generate_tasks(&mut self) -> Result<Vec<Task>> {
        let validator_tasks = self
            .validators
            .iter()
            .map(|v| Validator::from(*v).create_task());
        let collator_tasks = self
            .collators
            .iter()
            .map(|c| Collator::from(*c).create_task());

        validator_tasks.chain(collator_tasks).collect()
    }

    pub fn ensure_unique_ports(&self) -> Result<()> {
        let mut ports: HashSet<u16> = HashSet::new();

        fn check_node(ports: &mut HashSet<u16>, node: &impl Node) -> Result<()> {
            node.ports()
                .iter()
                .flatten()
                .try_for_each(|p| match ports.insert(*p) {
                    true => Ok(()),
                    false => Err(Error::PortInUse(*p)),
                })
        }

        let check_validator = |v| check_node(&mut ports, v);
        self.validators
            .iter()
            .map(|v| &Validator::from(*v))
            .try_for_each(check_validator)?;

        let check_collator = |c| check_node(&mut ports, c);
        self.collators
            .iter()
            .map(|c| &Collator::from(*c))
            .try_for_each(check_collator)
    }
}
