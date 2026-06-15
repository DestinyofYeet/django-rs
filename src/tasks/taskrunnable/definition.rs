use std::any::Any;

use crate::tasks::logstrategy::LogStrategyType;

pub trait TaskRunnable {
    fn run(&mut self, logger: LogStrategyType, worker_id: u64) -> Box<dyn Any + Send + Sync>;
}
