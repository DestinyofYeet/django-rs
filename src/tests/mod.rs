use std::path::PathBuf;

use crate::{
    server::{
        DjangoServer,
        database_strategy::{DatabaseStrategy, default_strategies::SqliteStrategy},
    },
    tasks::logstrategy::default_strategies::tracing_strategy::TracingStrategy,
};

mod example;
mod sqlite;

fn get_test_dir() -> PathBuf {
    let tempfile = tempfile::TempDir::new().expect("to get temp dir");
    tempfile.keep()
}

pub fn setup_server<D>(strategy: D) -> DjangoServer<D>
where
    D: DatabaseStrategy,
{
    DjangoServer::new(1, TracingStrategy {}, strategy).expect("to create server")
}

pub fn setup_sqlite_server() -> DjangoServer<SqliteStrategy> {
    let dir = get_test_dir();
    setup_server(SqliteStrategy::new(
        dir.join("database.db").to_str().unwrap(),
    ))
}
