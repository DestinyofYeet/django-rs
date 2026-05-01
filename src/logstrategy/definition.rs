use std::sync::Arc;

pub type LogStrategyType = Arc<dyn LogStrategy + Send + Sync>;

pub trait LogStrategy {
    fn warn(&self, worker_id: u64, message: &str);
    fn error(&self, worker_id: u64, message: &str);
    fn info(&self, worker_id: u64, message: &str);
}
