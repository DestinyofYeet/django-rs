use std::sync::Arc;

use crate::logstrategy::LogStrategy;

pub trait TaskRunnable {
    fn run(&mut self, logger: Arc<dyn LogStrategy>, worker_id: u64);
}
