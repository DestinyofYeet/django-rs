use std::sync::{Arc, Mutex};

use crate::tasks::{
    task::{Runnable, Task},
    taskhandler::{TaskEvent, TaskHandler, TaskHandlerError},
    taskref::TaskRef,
    taskrunnable::{TaskResultable, TaskRunnable},
};

impl TaskHandler {
    pub fn spawn_task<T>(&self, runnable: T) -> Result<TaskRef<T>, TaskHandlerError>
    where
        T: TaskResultable + TaskRunnable + Send + Sync + 'static,
    {
        let runnable: Runnable = Box::new(runnable);
        let task = Arc::new(Mutex::new(Task::new(runnable, self.log_strategy.clone())));

        let task_ref = TaskRef::new(task.clone());
        self.to_handler.send(TaskEvent::ProcessTask(task))?;
        Ok(task_ref)
    }
}
