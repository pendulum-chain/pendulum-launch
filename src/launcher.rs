use crate::{
    node::{Collator, Node, Validator},
    task::{Task, TaskManager},
    Config, Error, PathBuffer, Result,
};
use lazy_static::lazy_static;
use std::{
    collections::HashSet,
    path::PathBuf,
    sync::{Arc, RwLock},
};

lazy_static! {
    pub(crate) static ref LOG_DIR: Arc<RwLock<Option<PathBuffer>>> = Arc::new(RwLock::new(None));
}

#[derive(Debug)]
pub struct Launcher {
    pub name: Option<String>,
    pub author: Option<String>,
    pub validators: Vec<Validator>,
    pub collators: Vec<Collator>,
}

impl<'a> Launcher {
    pub fn new(config: Config, log_dir: Option<PathBuf>) -> Result<Self> {
        // Initialize LOG_DIR
        *Arc::clone(&LOG_DIR).write()? = log_dir.map(PathBuffer::from);

        let name = config.name.to_owned();
        let author = config.author.to_owned();
        let validators = config.validators.into_iter().map(Validator::from).collect();
        let collators = config.collators.into_iter().map(Collator::from).collect();

        Ok(Self {
            name,
            author,
            validators,
            collators,
        })
    }

    // Launches nodes and awaits termination
    pub fn run(&mut self) -> Result<()> {
        let tasks = self.generate_tasks()?;
        let mut task_manager = TaskManager::new(tasks);

        task_manager.run()
    }

    pub fn generate_tasks(&mut self) -> Result<Vec<Task>> {
        let validator_tasks = self.validators.iter().map(|v| v.create_task());
        let collator_tasks = self.collators.iter().map(|c| c.create_task());

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
        self.validators.iter().try_for_each(check_validator)?;

        let check_collator = |c| check_node(&mut ports, c);
        self.collators.iter().try_for_each(check_collator)
    }
}
