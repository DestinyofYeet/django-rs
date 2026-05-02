use thiserror::Error;

use crate::tasks::taskhandler::TaskError;

#[derive(Error, Debug)]
pub enum ServerError {
    #[error("{0}")]
    TaskHandler(#[from] TaskError),
}
