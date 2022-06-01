use super::Task;
use crate::Result;
use std::{
    sync::{Arc, Condvar, Mutex},
    time::{Duration, Instant},
};

#[derive(Debug)]
pub struct TaskManager {
    tasks: Vec<Task>,
    start_time: Instant,
}

impl<'a> TaskManager {
    #[inline]
    pub fn new(tasks: Vec<Task>) -> Self {
        Self {
            tasks,
            start_time: Instant::now(),
        }
    }

    // TODO: periodically output uptime
    #[inline]
    #[allow(unused)]
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

        if let Err(err) = self.start() {
            self.shutdown()?;
            return Err(err);
        }

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
