use std::sync::Arc;

use crate::{
    server::{ServerError, database_strategy::DatabaseStrategy},
    tasks::{logstrategy::LogStrategy, taskhandler::TaskHandler},
};

pub struct Server<D>
where
    D: DatabaseStrategy,
{
    task_handler: TaskHandler,
    database_strategy: Arc<D>,
}

impl<D> Server<D>
where
    D: DatabaseStrategy,
{
    pub fn new(
        workers: u64,
        logging_strategy: impl LogStrategy + Send + Sync + 'static,
        database_strategy: D,
    ) -> Result<Self, ServerError> {
        Ok(Self {
            task_handler: TaskHandler::new(workers, logging_strategy)?,
            database_strategy: Arc::new(database_strategy),
        })
    }

    pub fn get_database(&self) -> Arc<D> {
        self.database_strategy.clone()
    }

    pub fn get_task_handler(&self) -> &TaskHandler {
        &self.task_handler
    }

    pub fn shutdown(self) -> Result<(), ServerError> {
        self.task_handler.shutdown()?;
        Ok(())
    }
}
