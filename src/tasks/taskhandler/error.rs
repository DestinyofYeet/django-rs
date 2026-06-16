use std::{any::Any, sync::mpsc::SendError};

use thiserror::Error;

use crate::tasks::taskhandler::TaskEvent;

#[derive(Debug, Error)]
pub enum TaskHandlerError {
    #[error("Failed to send message: {0}")]
    SendError(#[from] SendError<TaskEvent>),

    #[error("Failed to join on thread")]
    Join,
}
