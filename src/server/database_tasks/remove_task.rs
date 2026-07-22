use std::{any::Any, marker::PhantomData, sync::Arc};

use crate::{
    models::{search::SearchQuery, traits::model::Model},
    server::database_strategy::DatabaseStrategy,
    tasks::{taskrunnable::TaskRunnable, worker_logger::WorkerLogger},
};

pub struct RemoveModelTask<D, M>
where
    D: DatabaseStrategy,
    M: Model,
{
    db: Arc<D>,
    search: SearchQuery,
    marker: PhantomData<M>,
}

impl<D, M> RemoveModelTask<D, M>
where
    D: DatabaseStrategy,
    M: Model,
{
    pub fn new(db: Arc<D>, search: SearchQuery) -> Box<Self> {
        Box::new(Self {
            db,
            search,
            marker: PhantomData,
        })
    }
}

impl<D, M> TaskRunnable for RemoveModelTask<D, M>
where
    D: DatabaseStrategy,
    M: Model,
{
    fn run(&mut self, _logger: WorkerLogger) -> Box<dyn Any + Send + Sync> {
        let conn = self.db.get_connection();
        Box::new(self.db.remove_model::<M>(&conn, &self.search))
    }
}
