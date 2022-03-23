use crate::error::Result;
use std::{
    cell::RefCell,
    process::{self, Command, ExitStatus},
};

#[derive(Debug)]
pub struct Task {
    command: Command,
    handle: RefCell<Option<process::Child>>,
}

impl Task {
    pub const fn new(command: Command) -> Self {
        Self {
            command,
            handle: RefCell::new(None),
        }
    }

    /// Executes the task, waiting before returing the `ExitStatus`
    pub fn execute(&mut self) -> Result<ExitStatus> {
        Ok(self.command.status()?)
    }

    /// Executes the task, spawing the process on a separate thread
    /// returning a handle
    pub fn spawn(&mut self) -> Result<()> {
        let handle = self.command.spawn()?;
        // if self.log_path.is_some() {
        //     self.init_logger()?;
        // }

        self.handle.replace(Some(handle));
        Ok(())
    }

    /// Kills the task if it's running
    pub fn kill(&mut self) -> Result<()> {
        if let Some(handle) = self.handle.get_mut() {
            handle.kill()?;
            self.handle.replace(None);
        }

        Ok(())
    }
}

impl From<String> for Task {
    fn from(command: String) -> Self {
        Self::new(Command::new(command))
    }
}
