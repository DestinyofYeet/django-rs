use uuid::Uuid;

use crate::{logstrategy::LogStrategyType, taskrunnable::TaskRunnable};

pub type Runnable = Box<dyn TaskRunnable + Sync + Send>;

pub struct Task {
    id: Uuid,
    runnable: Runnable,
    logger: LogStrategyType,
}

impl Task {
    pub fn new(runnable: Runnable, logger: LogStrategyType) -> Self {
        Self {
            id: Uuid::new_v4(),
            runnable,
            logger,
        }
    }

    pub fn run(&mut self, worker_id: u64) {
        self.runnable.run(self.logger.clone(), worker_id);
    }

    #[inline(always)]
    pub fn get_id(&self) -> Uuid {
        self.id
    }
}
