use crate::error::{Error, Result, SerdeError};
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};

mod collator;
mod validator;

pub use collator::CollatorConfig;
pub use validator::ValidatorConfig;

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub name: Option<String>,
    pub author: Option<String>,
    pub mode: Option<String>,
    pub validator: ValidatorConfig,
    pub collator: CollatorConfig,
}

impl Config {
    pub fn new(
        name: Option<&str>,
        author: Option<&str>,
        mode: Option<&str>,
        validator: ValidatorConfig,
        collator: CollatorConfig,
    ) -> Self {
        let to_string = |value: &str| value.to_string();

        Self {
            name: name.map(to_string),
            author: author.map(to_string),
            mode: mode.map(to_string),
            validator,
            collator,
        }
    }

    pub fn deserialize(path: PathBuf) -> Result<Self> {
        let raw_config = &fs::read(path)?;
        match serde_json::from_slice(raw_config) {
            Ok(config) => Ok(config),
            Err(err) => Err(Error::Serde(SerdeError::Deserialize(err.to_string()))),
        }
    }
}
