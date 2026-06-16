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
    tests::setup_sqlite_server,
};

use std::sync::LazyLock;

use chrono::{DateTime, Utc};
use django_rs_macro::{FromIter, SaveData};
use serde::{Deserialize, Serialize};

use crate::server::database_strategy::default_strategies::SqliteStrategy;

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
    let server = setup_sqlite_server();
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
    let server = setup_sqlite_server();
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

    let task = task_handler.spawn_task(save_task).unwrap();

    task_handler.wait_until_done(&task).unwrap();

    assert_eq!(task.get_state(), TaskState::Done);

    let result = task.get_result();

    let get_task = GetModelTask::<SqliteStrategy, TestModel>::new(
        db.clone(),
        SearchQuery::empty().add_constraint(("id", result.unwrap())),
    );

    let task = task_handler.spawn_task(get_task).unwrap();

    task_handler.wait_until_done(&task).unwrap();

    assert_eq!(task.get_state(), TaskState::Done);

    server.shutdown().unwrap();
}
