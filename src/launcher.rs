use crate::{error::Result, Config, Task};
use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread,
    time::{Duration, Instant},
};

#[derive(Debug)]
pub struct Launcher {
    tasks: Vec<Task>,
    start_time: Instant,
    pub active: Arc<AtomicBool>,
}

impl<'a> Launcher {
    pub fn new(config: Config) -> Self {
        let tasks = config.generate_tasks();

        Self {
            tasks,
            start_time: Instant::now(),
            active: Arc::new(AtomicBool::new(true)),
        }
    }

    #[inline]
    pub fn uptime(&self) -> Duration {
        self.start_time.elapsed()
    }

    pub fn run(&mut self) -> Result<()> {
        self.start()?;

        let active = Arc::clone(&self.active);
        while active.load(Ordering::Relaxed) {
            thread::sleep(Duration::from_millis(50));
        }

        self.shutdown()
    }

    fn start(&mut self) -> Result<()> {
        let active = Arc::clone(&self.active);
        ctrlc::set_handler(move || active.store(false, Ordering::Relaxed))?;

        self.tasks
            .iter_mut()
            .map(|task| task.spawn())
            .collect::<Result<()>>()
    }

    fn shutdown(&mut self) -> Result<()> {
        self.tasks
            .iter_mut()
            .map(|task| task.kill())
            .collect::<Result<()>>()
    }
}
