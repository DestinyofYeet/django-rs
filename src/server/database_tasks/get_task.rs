use std::{any::Any, marker::PhantomData, sync::Arc};

use crate::{
    models::{
        search::SearchQuery,
        traits::{from_iter::FromIter, model::Model},
    },
    server::database_strategy::{DatabaseStrategy, DatabaseStrategyError},
    tasks::{
        taskrunnable::{TaskResultable, TaskRunnable},
        worker_logger::WorkerLogger,
    },
};

pub struct GetModelTask<'a, D, M>
where
    D: DatabaseStrategy + 'a,
    M: Model,
{
    db: Arc<D>,
    search: SearchQuery,
    _m: PhantomData<&'a M>,
}

impl<'a, D, M> GetModelTask<'a, D, M>
where
    D: DatabaseStrategy,
    M: Model,
{
    pub fn new(db: Arc<D>, search: SearchQuery) -> Self {
        Self {
            search,
            db,
            _m: PhantomData,
        }
    }
}

impl<'a, D, M> TaskRunnable for GetModelTask<'a, D, M>
where
    D: DatabaseStrategy,
    M: Model + FromIter + Send + Sync + 'static,
{
    fn run(&mut self, _logger: WorkerLogger) -> Box<dyn Any + Send + Sync> {
        let result = self
            .db
            .search_single_model::<M>(&self.db.get_connection(), self.search.clone());

        Box::new(result)
    }
}

impl<'a, D, M> TaskResultable for GetModelTask<'a, D, M>
where
    D: DatabaseStrategy,
    M: Model + FromIter + 'static,
{
    type Result = Result<Option<M>, DatabaseStrategyError>;

    fn downcast(result: crate::tasks::task::TaskResult) -> Self::Result {
        *result.downcast().expect("to downcast")
    }
}
