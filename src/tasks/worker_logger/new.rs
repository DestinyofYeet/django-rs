use crate::tasks::{logstrategy::LogStrategyType, worker_logger::WorkerLogger};

impl WorkerLogger {
    pub fn new(logger: LogStrategyType, worker_id: u64) -> Self {
        Self { logger, worker_id }
    }
}
