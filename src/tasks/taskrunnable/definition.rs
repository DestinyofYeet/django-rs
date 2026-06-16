use std::any::Any;

use crate::tasks::{logstrategy::LogStrategyType, task::TaskResult};

pub trait TaskRunnable {
    fn run(&mut self, logger: LogStrategyType, worker_id: u64) -> Box<dyn Any + Send + Sync>;
}

pub trait TaskResultable {
    type Result;

    fn downcast(result: TaskResult) -> Self::Result;
}
