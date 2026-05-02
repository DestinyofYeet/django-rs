use std::path::PathBuf;

use sqlite::Connection;

use crate::{
    models::{Model, ModelValueType},
    server::database_strategy::{DatabaseStrategy, DatabaseStrategyError},
};

pub struct SqliteStrategy {
    conn: Connection,
}

impl SqliteStrategy {
    pub fn new(path: impl ToString) -> Self {
        let sqlite = sqlite::open(path.to_string()).unwrap();

        Self { conn: sqlite }
    }
}

impl DatabaseStrategy for SqliteStrategy {
    fn migrate_model(&self, model: impl Model) -> Result<(), DatabaseStrategyError> {
        let data = model.get_fields();
        let latest = &data[0];
        struct ResultingTable {
            table_name: String,
            fields: Vec<(String, ModelValueType)>,
        }

        let mut table = ResultingTable {
            table_name: latest.model_name.clone(),
            fields: Vec::new(),
        };

        for field in latest.data.iter() {
            table.fields.push((field.key.clone(), field.value));
        }

        let query = "select name from sqlite_master where type = 'table' and name = :name";
        let mut statement = self.conn.prepare(query).unwrap();
        statement
            .bind((":name", table.table_name.as_str()))
            .map_err(|e| DatabaseStrategyError::MigrateModel(e.to_string()))?;

        let table_exists = statement.column_count() != 0;

        println!("table_exists: {table_exists}");

        Ok(())
    }
}
