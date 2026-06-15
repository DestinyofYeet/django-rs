use std::{any::Any, marker::PhantomData, sync::Arc};

use crate::{
    models::{
        search::SearchQuery,
        traits::{from_iter::FromIter, model::Model},
    },
    server::database_strategy::{DatabaseStrategy, DatabaseStrategyError},
    tasks::taskrunnable::TaskRunnable,
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
    pub fn new(db: Arc<D>, search: SearchQuery) -> Box<Self> {
        Box::new(Self {
            search,
            db,
            _m: PhantomData,
        })
    }
}

impl<'a, D, M> TaskRunnable for GetModelTask<'a, D, M>
where
    D: DatabaseStrategy,
    M: Model + FromIter + 'static + Send + Sync,
{
    fn run(
        &mut self,
        logger: crate::tasks::logstrategy::LogStrategyType,
        worker_id: u64,
    ) -> Box<dyn Any + Send + Sync> {
        let result = self
            .db
            .search_single_model::<M>(&self.db.get_connection(), self.search.clone());

        Box::new(result)
    }

    // fn downcast<B>(result: Box<dyn Any + Send + Sync>) -> Box<Result<M, DatabaseStrategyError>> {
    //     let result = result
    //         .downcast::<Result<M, DatabaseStrategyError>>()
    //         .unwrap();

    //     result
    // }

    // fn downcast<Box<Result<MyType, SomeError>>>(result: Box<dyn Any + Send + Sync>) -> Box<Result<MyType, SomeError>> {
    //     let result = result
    //         .downcast::<Result<M, DatabaseStrategyError>>()
    //         .unwrap();
    // }
}
