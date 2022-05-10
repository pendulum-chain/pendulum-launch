use crate::{error::Result, Config, PathBuffer, Task};
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
pub enum LauncherMode {
    Local,
    TestNet,
}

#[derive(Debug)]
pub struct Launcher {
    tasks: Vec<Task>,
    start_time: Instant,
    //TODO: remove this
    #[allow(dead_code)]
    mode: LauncherMode,
}
impl<'a> Launcher {
    #[inline]
    pub fn new(config: &mut Config, log_dir: Option<PathBuf>) -> Result<Self> {
        // Initialize LOG_DIR
        *Arc::clone(&LOG_DIR).write()? = log_dir.map(PathBuffer::from);

        let mut mode = LauncherMode::Local;
        if let Some(launcher_mode) = &config.mode {
            mode = match launcher_mode.to_lowercase().as_str() {
                "local" => LauncherMode::Local,
                "testnet" => LauncherMode::TestNet,
                _ => LauncherMode::Local
            }
        }

        Ok(Self {
            tasks: config.generate_tasks()?,
            start_time: Instant::now(),
            mode,
        })
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
