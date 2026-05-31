use django_rs::chrono::format::Parsed;
use django_rs::models::save::SaveModel;
use django_rs::{
    django_rs_macro::{FromIter, SaveData},
    models::{
        ModelIteration,
        column::{ColumnType, CreateColumn, CreateOptions},
        traits::model::Model,
    },
    server::{
        Server,
        database_strategy::{DatabaseStrategy, default_strategies::SqliteStrategy},
    },
    tasks::logstrategy::default_strategies::tracing_strategy::TracingStrategy,
};

#[derive(Debug, FromIter, SaveData)]
pub struct Test {
    id: Option<i32>,
    key: String,
    value: i32,
}

impl Model for Test {
    const TABLE_NAME: &'static str = "test";

    fn get_migration() -> Vec<django_rs::models::ModelIteration> {
        vec![ModelIteration::Create(vec![
            CreateColumn::new(
                "id",
                ColumnType::Integer,
                CreateOptions::default().set_primary_key(),
            ),
            CreateColumn::new("key", ColumnType::String, CreateOptions::default()),
            CreateColumn::new("value", ColumnType::Integer, CreateOptions::default()),
        ])]
    }

    fn get_id(&self) -> Option<i64> {
        self.id.map(|e| e as i64)
    }

    fn set_id(&mut self, id: i64) {
        self.id = Some(id as i32)
    }
}

fn main() {
    let server = Server::new(8, TracingStrategy {}, SqliteStrategy::new("./test.db")).unwrap();

    let db = server.get_database();

    let mut test = Test {
        id: None,
        key: "test".to_string(),
        value: 8,
    };

    db.migrate_model::<Test>().unwrap();

    db.save_model(&db.get_connection(), &mut test).unwrap();

    server.shutdown().unwrap()
}
