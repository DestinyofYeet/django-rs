use std::any::Any;

use uuid::Uuid;

use crate::tasks::{
    logstrategy::LogStrategyType, taskrunnable::TaskRunnable, worker_logger::WorkerLogger,
};

pub type Runnable = Box<dyn TaskRunnable + Sync + Send>;

pub type TaskResult = Box<dyn Any + Send + Sync>;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum TaskState {
    Queued,
    Running,
    Done,
}

pub(crate) struct Task {
    id: Uuid,
    runnable: Runnable,
    logger: LogStrategyType,
    state: TaskState,
    result: Option<TaskResult>,
}

impl Task {
    pub(crate) fn new(runnable: Runnable, logger: LogStrategyType) -> Self {
        Self {
            id: Uuid::new_v4(),
            runnable,
            logger,
            state: TaskState::Queued,
            result: None,
        }
    }

    pub(crate) fn run(&mut self, worker_id: u64) -> TaskResult {
        let logger = WorkerLogger::new(self.logger.clone(), worker_id);
        self.runnable.run(logger)
    }

    pub(crate) fn set_result(&mut self, result: TaskResult) {
        self.result = Some(result);
    }

    pub(crate) fn get_result(&mut self) -> Option<TaskResult> {
        self.result.take()
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
