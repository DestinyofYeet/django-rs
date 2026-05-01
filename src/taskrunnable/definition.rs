use crate::logstrategy::LogStrategyType;

pub trait TaskRunnable {
    fn run(&mut self, logger: LogStrategyType, worker_id: u64);
}
