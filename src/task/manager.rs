use crate::{Config, PathBuffer, Result, Task};
use lazy_static::lazy_static;
use std::{
    path::PathBuf,
    sync::{Arc, Condvar, Mutex, RwLock},
    time::{Duration, Instant},
};

lazy_static! {
    pub(crate) static ref LOG_DIR: Arc<RwLock<Option<PathBuffer>>> = Arc::new(RwLock::new(None));
}

#[derive(Debug)]
pub struct TaskManager {
    tasks: Vec<Task>,
    start_time: Instant,
}

impl<'a> TaskManager {
    #[inline]
    pub fn new(tasks: Vec<Task>, log_dir: Option<PathBuf>) -> Self {
        Self {
            tasks,
            start_time: Instant::now(),
        }
    }

    #[inline]
    pub fn uptime(&self) -> Duration {
        self.start_time.elapsed()
    }

    pub fn run(&mut self) -> Result<()> {
        // Flag for validating completion of tasks
        let finished_pair = Arc::new((Mutex::new(false), Condvar::new()));

        // Listen for SIGINT, setting the finish flag and notifying the condition variable upon
        // receival
        let finished_pair_clone = Arc::clone(&finished_pair);
        let sig_handler = move || -> std::result::Result<(), ctrlc::Error> {
            let (lock, cvar) = &*finished_pair_clone;
            *lock.lock()? = true;
            cvar.notify_one();
            Ok(())
        };
        ctrlc::set_handler(sig_handler)?;

        self.start()?;

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
