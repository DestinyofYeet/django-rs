use thiserror::Error;

#[derive(Error, Debug)]
pub enum WorkerError {
    #[error("Failed to send message: {0}")]
    Channel(String),

    #[error("Failed to join on thread")]
    Join,
}
