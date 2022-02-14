use crate::{error::Result, Config, PathBuffer, Task};
use std::{
    path::PathBuf,
    sync::{Arc, Condvar, Mutex},
    time::{Duration, Instant},
};

#[derive(Debug)]
pub struct Launcher {
    tasks: Vec<Task>,
    start_time: Instant,
}
impl<'a> Launcher {
    #[inline]
    pub fn new(config: &mut Config, log_dir: Option<PathBuf>) -> Result<Self> {
        Ok(Self {
            tasks: config.generate_tasks(log_dir.map(PathBuffer::from))?,
            start_time: Instant::now(),
        })
    }

    #[inline]
    pub fn uptime(&self) -> Duration {
        self.start_time.elapsed()
    }

    pub fn run(&mut self) -> Result<()> {
        // Flag for validating completion of tasks
        let finished_pair = Arc::new((Mutex::new(false), Condvar::new()));

        self.start()?;

        // Listen for SIGINT, setting the finish flag and notifying the condition variable upon
        // receival
        let finished_pair_clone = Arc::clone(&finished_pair);
        ctrlc::set_handler(move || {
            let (lock, cvar) = &*finished_pair_clone;
            *lock.lock().unwrap() = true;
            cvar.notify_one();
        })?;

        // Wait for the thread to finish
        let (lock, cvar) = &*finished_pair;
        let mut started = lock.lock().unwrap();
        while !*started {
            started = cvar.wait(started).unwrap();
        }

        self.shutdown()
    }

    fn start(&mut self) -> Result<()> {
        self.tasks.iter_mut().try_for_each(|task| task.spawn())
    }

    fn shutdown(&mut self) -> Result<()> {
        self.tasks.iter_mut().try_for_each(|task| task.kill())
    }
}
