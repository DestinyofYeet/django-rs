use crate::{
    self as django_rs,
    models::{
        MigrationKind, ModelMigration,
        column::{ColumnType, CreateColumn, CreateOptions},
        search::SearchQuery,
        traits::model::Model,
    },
    server::{
        database_strategy::DatabaseStrategy,
        database_tasks::{GetModelTask, SaveModelTask},
    },
    tasks::task::TaskState,
};

use std::{path::PathBuf, sync::LazyLock, thread, time::Duration};

use chrono::{DateTime, Utc};
use django_rs_macro::{FromIter, SaveData};
use serde::{Deserialize, Serialize};

use crate::{
    server::{Server, database_strategy::default_strategies::SqliteStrategy},
    tasks::logstrategy::default_strategies::tracing_strategy::TracingStrategy,
};

fn get_test_dir() -> PathBuf {
    let tempfile = tempfile::TempDir::new().expect("to get temp dir");
    tempfile.keep()
}

fn setup_server() -> Server<SqliteStrategy> {
    let dir = get_test_dir();

    Server::new(
        1,
        TracingStrategy {},
        SqliteStrategy::new(dir.join("database.db").to_str().unwrap()),
    )
    .expect("to create server")
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Data {
    One(String),
    Two,
}

#[derive(Debug, SaveData, FromIter)]
pub struct TestModel {
    id: Option<i64>,
    name: String,
    created_at: DateTime<Utc>,
    extra_data: Data,
}

impl Model for TestModel {
    const TABLE_NAME: &'static str = "TestModel";

    fn get_migration() -> &'static Vec<crate::models::ModelMigration> {
        static MIGRATIONS: LazyLock<Vec<ModelMigration>> = LazyLock::new(|| {
            vec![ModelMigration::new(
                0,
                MigrationKind::Create(vec![
                    CreateColumn::new(
                        "id",
                        ColumnType::Integer,
                        CreateOptions::default().set_primary_key(),
                    ),
                    CreateColumn::new(
                        "name",
                        ColumnType::String,
                        CreateOptions::default().set_non_nullable(),
                    ),
                    CreateColumn::new(
                        "created_at",
                        ColumnType::Date,
                        CreateOptions::default().set_non_nullable(),
                    ),
                    CreateColumn::new(
                        "extra_data",
                        ColumnType::Json,
                        CreateOptions::default().set_non_nullable(),
                    ),
                ]),
            )]
        });

        &MIGRATIONS
    }

    fn get_id(&self) -> Option<i64> {
        self.id
    }

    fn set_id(&mut self, id: i64) {
        self.id = Some(id)
    }
}

#[test]
pub fn test_save_and_retrieve() {
    let server = setup_server();
    let db = server.get_database();

    db.migrate_model::<TestModel>().unwrap();

    let mut model = TestModel {
        id: None,
        name: "some_name".to_string(),
        created_at: Utc::now(),
        extra_data: Data::One("weeee".to_string()),
    };

    db.save_model(&db.get_connection(), &mut model).unwrap();

    db.search_single_model::<TestModel>(
        &db.get_connection(),
        SearchQuery::empty().add_constraint(("id", model.id.unwrap())),
    )
    .unwrap()
    .unwrap();

    server.shutdown().unwrap();
}

#[test]
pub fn test_save_and_retrieve_task() {
    let server = setup_server();
    let task_handler = server.get_task_handler();
    let db = server.get_database();

    db.migrate_model::<TestModel>().unwrap();

    let model = TestModel {
        id: None,
        name: "some_name".to_string(),
        created_at: Utc::now(),
        extra_data: Data::One("weeee".to_string()),
    };

    let save_task = SaveModelTask::new(db.clone(), model);

    let task = task_handler.spawn_task(save_task);

    task_handler.wait_until_done(task.clone());

    assert_eq!(
        task.lock().expect("to get lock").get_state(),
        TaskState::Done
    );

    let get_task = GetModelTask::<SqliteStrategy, TestModel>::new(
        db.clone(),
        SearchQuery::empty().add_constraint(("id", model.id.unwrap())),
    );

    let task = task_handler.spawn_task(get_task);
    task_handler.wait_until_done(task.clone());

    assert_eq!(
        task.lock().expect("to get lock").get_state(),
        TaskState::Done
    );

    server.shutdown().unwrap();
}
