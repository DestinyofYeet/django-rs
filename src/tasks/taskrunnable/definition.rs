use std::any::Any;

use crate::tasks::{task::TaskResult, worker_logger::WorkerLogger};

pub trait TaskRunnable {
    fn run(&mut self, logger: WorkerLogger) -> Box<dyn Any + Send + Sync>;
}

pub trait TaskResultable {
    type Result;

    fn downcast(result: TaskResult) -> Self::Result;
}
