use std::sync::Arc;

use crate::{
    models::traits::{
        from_iter::FromIter,
        model::Model,
        save_data::{SaveData, ValidateSaveData},
    },
    server::database_strategy::DatabaseStrategy,
    tasks::taskrunnable::TaskRunnable,
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
    pub fn new(db: Arc<D>, model: M) -> Box<Self> {
        Box::new(Self { db, model })
    }
}

impl<D, M> TaskRunnable for SaveModelTask<D, M>
where
    D: DatabaseStrategy,
    M: Model + SaveData + FromIter + ValidateSaveData,
{
    fn run(&mut self, logger: crate::tasks::logstrategy::LogStrategyType, worker_id: u64) {
        let conn = self.db.get_connection();
        match self.db.save_model(&conn, &mut self.model) {
            Ok(_) => {}
            Err(e) => logger.error(worker_id, &format!("Failed to save model: {e}")),
        }
    }
}
