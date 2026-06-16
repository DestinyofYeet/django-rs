use thiserror::Error;

use crate::tasks::taskhandler::TaskHandlerError;

#[derive(Error, Debug)]
pub enum ServerError {
    #[error("{0}")]
    TaskHandler(#[from] TaskHandlerError),
}
