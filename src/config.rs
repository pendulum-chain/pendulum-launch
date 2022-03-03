use crate::{
    error::{Error, Result, SerdeError},
    node::{Collator, Node, Validator},
    Task,
};
use serde::{Deserialize, Serialize};
use std::{collections::HashSet, fs, path::PathBuf};

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

    pub fn deserialize(path: PathBuf) -> Result<Self> {
        let raw_config = &fs::read(path)?;
        match serde_json::from_slice(raw_config) {
            Ok(config) => Ok(config),
            Err(err) => Err(Error::Serde(SerdeError::Deserialize(err.to_string()))),
        }
    }

    pub fn generate_tasks(&mut self) -> Result<Vec<Task>> {
        let validator_tasks = self.validators.iter().map(|v| v.create_task());
        let collator_tasks = self.collators.iter().map(|c| c.create_task());
        validator_tasks.chain(collator_tasks).collect()
    }

    pub fn ensure_unique_ports(&self) -> Result<()> {
        let mut ports: HashSet<u16> = HashSet::new();

        fn check_single_node(ports: &mut HashSet<u16>, node: &impl Node) -> Result<()> {
            node.ports()
                .iter()
                .flatten()
                .try_for_each(|p| match ports.contains(p) {
                    true => Err(Error::PortInUse(*p)),
                    false => {
                        ports.insert(*p);
                        Ok(())
                    }
                })
        }

        self.validators
            .iter()
            .map(|n| check_single_node(&mut ports, n))
            .collect::<Result<()>>()?;

        self.collators
            .iter()
            .map(|n| check_single_node(&mut ports, n))
            .collect::<Result<()>>()?;

        Ok(())
    }
}
