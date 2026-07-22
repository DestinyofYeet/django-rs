use std::{any::Any, sync::Arc};

use crate::{
    models::traits::{
        from_iter::FromIter,
        model::Model,
        save_data::{SaveData, ValidateSaveData},
    },
    server::database_strategy::{DatabaseStrategy, DatabaseStrategyError},
    tasks::{
        taskrunnable::{TaskResultable, TaskRunnable},
        worker_logger::WorkerLogger,
    },
};

pub struct SaveModelTask<D, M>
where
    D: DatabaseStrategy,
    M: Model,
{
    db: Arc<D>,
    model: M,
}

impl<D, M> SaveModelTask<D, M>
where
    D: DatabaseStrategy,
    M: Model,
{
    pub fn new(db: Arc<D>, model: M) -> Self {
        Self { db, model }
    }

    pub fn get_model(&self) -> &M {
        &self.model
    }
}

impl<D, M> TaskRunnable for SaveModelTask<D, M>
where
    D: DatabaseStrategy,
    M: Model + SaveData + FromIter + ValidateSaveData + Send + Sync,
{
    fn run(&mut self, logger: WorkerLogger) -> Box<dyn Any + Send + Sync> {
        let conn = self.db.get_connection();
        match self.db.save_model(&conn, &mut self.model) {
            Ok(_) => {}
            Err(e) => logger.error(&format!("Failed to save model: {e}")),
        };
        Box::new(self.model.get_id())
    }
}

impl<D, M> TaskResultable for SaveModelTask<D, M>
where
    D: DatabaseStrategy,
    M: Model + SaveData + FromIter + ValidateSaveData,
{
    type Result = Option<i64>;

    fn downcast(result: crate::tasks::task::TaskResult) -> Self::Result {
        *result.downcast().expect("to parse result")
    }
}
