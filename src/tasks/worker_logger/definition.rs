use crate::tasks::logstrategy::LogStrategyType;

pub struct WorkerLogger {
    pub(crate) logger: LogStrategyType,
    pub(crate) worker_id: u64,
}
