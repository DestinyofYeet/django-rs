use itertools::Itertools;
use rusqlite::Connection;
use tracing::info;
use tracing_subscriber::fmt::init;

use crate::{
    models::{Model, ModelValueType},
    server::database_strategy::{DatabaseStrategy, DatabaseStrategyError},
};

pub struct SqliteStrategy {
    conn: Connection,
}

impl SqliteStrategy {
    pub fn new(path: impl ToString) -> Self {
        let sqlite = Connection::open(path.to_string()).unwrap();

        Self { conn: sqlite }
    }
}

impl DatabaseStrategy for SqliteStrategy {
    fn migrate_model<M: Model>(&self) -> Result<(), DatabaseStrategyError> {
        let migration_data = M::get_migration();

        if migration_data.data.len() != 1 {
            return Err(DatabaseStrategyError::MigrateModel(format!(
                "Migration for model {} needs to have at least one migration!",
                &migration_data.model_name
            )));
        }

        let table_exists = self.table_exists(&migration_data.model_name)?;

        println!("table_exists: {table_exists}");

        if !table_exists {
            let initial_fields = &migration_data.data[0];

            for field in initial_fields.data.iter() {
                match field.action {
                    ModelAction::Create(_) => {}
                    _ => {
                        return Err(DatabaseStrategyError::MigrateModel(format!(
                            "Field {}, needs to be a creation error.",
                            field.key
                        )));
                    }
                }
            }

            let sql = format!(
                "CREATE TABLE {} (\n {} \n)",
                migration_data.model_name,
                initial_fields
                    .data
                    .iter()
                    .map(|field| {
                        format!(
                            "\t {} {} {}",
                            field.key,
                            SqliteStrategy::match_model(&field.value),
                            SqliteStrategy::match_column_creation(&field.action)
                        )
                    })
                    .join(",\n")
            );

            println!("{sql}");

            self.conn
                .execute(&sql, [])
                .map_err(|e| DatabaseStrategyError::MigrateModel(e.to_string()))?;

            info!("Created table {}", &migration_data.model_name);
        }

        Ok(())
    }

    fn table_exists(&self, table_name: &str) -> Result<bool, DatabaseStrategyError> {
        self.conn
            .table_exists(None, table_name)
            .map_err(|e| DatabaseStrategyError::MigrateModel(e.to_string()))
    }

    fn match_model(value: &ModelValueType) -> String {
        (match value {
            ModelValueType::String => "TEXT",
            ModelValueType::Integer => "INTEGER",
            ModelValueType::Float => "REAL",
            ModelValueType::Date => "TEXT",
        })
        .to_string()
    }

    // fn match_action(value: &ModelAction) -> String {
    //     let mut output = String::new();

    //     match value {
    //         ModelAction::Create(model_create_options) => {
    //             if !model_create_options.nullable {
    //                 output.push_str("NOT NULL");
    //             }

    //             if model_create_options.primary_key {
    //                 output.push_str("PRIMARY KEY");
    //             }
    //         }
    //         ModelAction::RenameField { from: _, to: _ } => {
    //             panic!("Cannot use field rename in this context.")
    //         }
    //     }

    //     output
    // }
}
