use std::sync::Arc;

use uuid::Uuid;

use crate::{logstrategy::LogStrategy, taskrunnable::TaskRunnable};

pub type Runnable = Box<dyn TaskRunnable + Sync + Send>;

pub struct Task<T>
where
    T: LogStrategy,
{
    id: Uuid,
    runnable: Runnable,
    logger: Arc<T>,
}

impl<T> Task<T>
where
    T: LogStrategy + 'static,
{
    pub fn new(runnable: Runnable, logger: Arc<T>) -> Self {
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
