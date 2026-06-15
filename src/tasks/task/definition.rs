use std::any::Any;

use uuid::Uuid;

use crate::tasks::{logstrategy::LogStrategyType, taskrunnable::TaskRunnable};

pub type Runnable = Box<dyn TaskRunnable + Sync + Send>;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum TaskState {
    Queued,
    Running,
    Done,
}

pub struct Task {
    id: Uuid,
    runnable: Runnable,
    logger: LogStrategyType,
    state: TaskState,
}

impl Task {
    pub fn new(runnable: Runnable, logger: LogStrategyType) -> Self {
        Self {
            id: Uuid::new_v4(),
            runnable,
            logger,
            state: TaskState::Queued,
        }
    }

    pub(crate) fn run(&mut self, worker_id: u64) -> Box<dyn Any + Send + Sync> {
        self.runnable.run(self.logger.clone(), worker_id)
    }

    #[inline(always)]
    pub fn get_id(&self) -> Uuid {
        self.id
    }

    pub(crate) fn set_state(&mut self, state: TaskState) {
        self.state = state;
    }

    pub fn get_state(&self) -> TaskState {
        self.state
    }
}
