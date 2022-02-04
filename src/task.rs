use crate::error::{Error, Result};
use crate::PathBuffer;
use std::ffi::OsStr;
use std::io::{BufReader, BufWriter};
use std::process::Child;
use std::{cell::RefCell, process, process::ExitStatus};

#[derive(Debug)]
pub struct Task {
    command: process::Command,
    handle: RefCell<Option<process::Child>>,
    log_file: Option<PathBuffer>,
}

impl Task {
    pub fn new(command: process::Command, quiet: bool, log_file: Option<PathBuffer>) -> Self {
        Self {
            command,
            handle: RefCell::new(None),
            log_file,
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

    fn init_logging(&mut self) -> Result<()> {
        let handle = self.handle_mut()?;
        let stdout = match handle.stdout.take() {
            Some(stdout) => stdout,
            None => {
                return Err(Error::ProcessFailed(format!(
                    "could not initilize stdout for {:?}",
                    self.command.get_program()
                )))
            }
        };
        let stderr = match handle.stderr.take() {
            Some(stderr) => stderr,
            None => {
                return Err(Error::ProcessFailed(format!(
                    "could not initilize stderr for {:?}",
                    self.command.get_program()
                )))
            }
        };

        let log_file = match &self.log_file {
            Some(log_file) => log_file,
            None => return Err(Error::Uninitialized("Log file not declared".to_string())),
        };

        let log_file = std::fs::File::create(log_file.as_ref())?;

        BufWriter::new(log_file);

        BufReader::new(stdout);
        BufReader::new(stderr);

        Ok(())
    }

    fn handle_mut(&mut self) -> Result<&mut Child> {
        match self.handle.get_mut().as_mut() {
            Some(handle) => Ok(handle),
            None => Err(Error::Uninitialized(format!(
                "{:?} has not been spawned",
                self.command.get_program()
            ))),
        }
    }
}
