use std::sync::Arc;

use crate::{
    server::{ServerError, database_strategy::DatabaseStrategy},
    tasks::{logstrategy::LogStrategy, taskhandler::TaskHandler},
};

pub struct Server {
    task_handler: TaskHandler,
    database_strategy: Arc<dyn DatabaseStrategy>,
}

impl Server {
    pub fn new(
        workers: u64,
        logging_strategy: impl LogStrategy + Send + Sync + 'static,
        database_strategy: impl DatabaseStrategy + 'static,
    ) -> Result<Self, ServerError> {
        Ok(Self {
            task_handler: TaskHandler::new(workers, logging_strategy)?,
            database_strategy: Arc::new(database_strategy),
        })
    }

    pub fn get_database(&self) -> Arc<dyn DatabaseStrategy> {
        self.database_strategy.clone()
    }

    pub fn shutdown(self) -> Result<(), ServerError> {
        self.task_handler.shutdown()?;
        Ok(())
    }
}
