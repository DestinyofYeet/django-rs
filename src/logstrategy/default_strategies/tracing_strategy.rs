use tracing::{error, info, warn};

use crate::logstrategy::LogStrategy;

pub struct TracingStrategy {}

impl LogStrategy for TracingStrategy {
    fn warn(&self, worker_id: u64, message: &str) {
        warn!("[Worker {worker_id}] {message}");
    }

    fn error(&self, worker_id: u64, message: &str) {
        error!("[Worker {worker_id}] {message}");
    }

    fn info(&self, worker_id: u64, message: &str) {
        info!("[Worker {worker_id}] {message}");
    }
}
