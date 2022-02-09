use crate::{error::Result, PathBuffer};
use std::{
    cell::RefCell,
    // fs::File,
    // io::{BufReader, BufWriter, Read, Write},
    // process::{self, Child, ChildStderr, ChildStdout, ExitStatus},
    process::{self, ExitStatus},
};

// /// Wrapper around moved process stdout and stderr, for nesting in a task
// #[derive(Debug)]
// struct LogBuffer {
//     file: BufWriter<File>,
//     out: ChildStdout,
//     err: ChildStderr,
// }

// impl LogBuffer {
//     pub const fn new(file: File, out: ChildStdout, err: ChildStderr) -> Self {
//         Self {
//             file: BufWriter::new(file),
//             out,
//             err,
//         }
//     }

//     pub fn write(&mut self) -> Result<()> {
// let mut buf = Vec::new();
// self.out.read(&mut buf);
// self.err.read(&mut buf);

// self.file.write

// self.file.write(&buf);

// Ok(())
// }
// }

#[derive(Debug)]
pub struct Task {
    command: process::Command,
    handle: RefCell<Option<process::Child>>,
    // log_path: Option<PathBuffer>,
    // log_buffer: Option<LogBuffer>,
    // log: File,
}

impl Task {
    pub const fn new(command: process::Command) -> Self {
        Self {
            command,
            handle: RefCell::new(None),
            // log_path,
            // log_buffer: None,
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

    // pub fn log(&mut self) -> Result<()> {
    //     let buffer = match self.log_buffer.as_mut() {
    //         Some(buf) => buf,
    //         None => {
    //             return Err(Error::Uninitialized(format!(
    //                 "logger for {:?}",
    //                 self.command.get_program()
    //             )))
    //         }
    //     };

    //     Ok(())
    // }

    // /// Initializes inner LogBuffer
    // fn init_logger(&mut self) -> Result<()> {
    // let handle = self.handle_mut()?;
    // let stdout = match handle.stdout.take() {
    //     Some(stdout) => stdout,
    //     None => {
    //         return Err(Error::ProcessFailed(format!(
    //             "could not initilize stdout for {:?}",
    //             self.command.get_program()
    //         )))
    //     }
    // };
    // let stderr = match handle.stderr.take() {
    //     Some(stderr) => stderr,
    //     None => {
    //         return Err(Error::ProcessFailed(format!(
    //             "could not initilize stderr for {:?}",
    //             self.command.get_program()
    //         )))
    //     }
    // };

    // let log_path = match &self.log_path {
    //     Some(path) => path,
    //     None => {
    //         return Err(Error::Uninitialized(format!(
    //             "Log path not provided for {:?}",
    //             self.command.get_program()
    //         )))
    //     }
    // };

    // let log_file = std::fs::File::create(log_path.as_ref())?;
    // let out_buf = BufReader::new(stdout);
    // let err_buf = BufReader::new(stderr);
    // self.log_buffer = Some(LogBuffer::new(log_file, out_buf, err_buf));

    // Ok(())
    // }

    // fn handle_mut(&mut self) -> Result<&mut Child> {
    //     match self.handle.get_mut().as_mut() {
    //         Some(handle) => Ok(handle),
    //         None => Err(Error::Uninitialized(format!(
    //             "{:?} has not been spawned",
    //             self.command.get_program()
    //         ))),
    //     }
    // }
}
