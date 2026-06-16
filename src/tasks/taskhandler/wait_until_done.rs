use std::sync::mpsc::channel;

use crate::tasks::{
    taskhandler::{TaskEvent, TaskHandler, TaskHandlerError, TaskSubscriberEvent},
    taskref::TaskRef,
};

impl TaskHandler {
    pub fn wait_until_done<T>(&self, task: &TaskRef<T>) -> Result<(), TaskHandlerError> {
        let (tx, rx) = channel();

        self.to_handler.send(TaskEvent::RegisterSubscriber {
            for_task: task.get_id(),
            subscriber: tx,
        })?;

        while let Some(message) = rx.iter().next() {
            match message {
                TaskSubscriberEvent::CommInit => {}
                TaskSubscriberEvent::TaskDone => {
                    self.to_handler.send(TaskEvent::UnregisterSubscriber {
                        for_task: task.get_id(),
                    })?;
                    break;
                }
            }
        }

        Ok(())
    }
}
