use crate::tasks::taskhandler::{TaskEvent, TaskHandler, TaskHandlerError};

impl TaskHandler {
    pub(crate) fn shutdown(self) -> Result<(), TaskHandlerError> {
        self.to_handler.send(TaskEvent::Shutdown)?;
        self.handle.join().map_err(|_| TaskHandlerError::Join)?;
        Ok(())
    }
}
