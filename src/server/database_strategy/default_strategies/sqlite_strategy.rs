use itertools::Itertools;
use rusqlite::{Connection, params};
use tracing::{info, trace};

use crate::{
    models::{
        ColumnType, Model, ModelIteration,
        column::{CreateColumnOptions, CreateColumnOptionsValues},
    },
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

        if migration_data.data.is_empty() {
            return Err(DatabaseStrategyError::MigrateModel(format!(
                "Migration for model {} needs to have at least one migration!",
                &migration_data.model_name
            )));
        }

        for (count, migration) in migration_data.data.iter().enumerate() {
            match migration {
                ModelIteration::Create(columns) => {
                    if count != 0 {
                        return Err(DatabaseStrategyError::MigrateModel(format!(
                            "Can only create a table at the first iteration, not at iteration {count}"
                        )));
                    }

                    if self.table_exists(&migration_data.model_name)? {
                        continue;
                    }

                    let mut sql = format!("CREATE TABLE {} (\n", migration_data.model_name);
                    sql += &columns
                        .iter()
                        .map(|col| {
                            format!(
                                "    {} {} {}",
                                col.key,
                                SqliteStrategy::match_column_type(&col.value),
                                SqliteStrategy::match_create_column_options(&col.options, &col.key)
                            )
                        })
                        .join(",\n");

                    sql += "\n)";

                    self.conn
                        .execute(&sql, [])
                        .map_err(|e| DatabaseStrategyError::MigrateModel(e.to_string()))?;

                    trace!("produced sql: {sql}");
                    info!("Created table {}", migration_data.model_name);
                }
                ModelIteration::Modify => todo!(),
            }
        }

        Ok(())
    }

    fn table_exists(&self, table_name: &str) -> Result<bool, DatabaseStrategyError> {
        self.conn
            .table_exists(None, table_name)
            .map_err(|e| DatabaseStrategyError::MigrateModel(e.to_string()))
    }

    fn match_column_type(value: &ColumnType) -> String {
        (match value {
            ColumnType::String => "TEXT",
            ColumnType::Integer => "INTEGER",
            ColumnType::Float => "REAL",
            ColumnType::Date => "TEXT",
        })
        .to_string()
    }

    fn match_create_column_options(value: &CreateColumnOptions, _: &str) -> String {
        let mut options = Vec::<String>::new();

        for option in value.options.iter() {
            match option {
                CreateColumnOptionsValues::Nullable => {
                    options.push("NOT NULL".to_string());
                }
                CreateColumnOptionsValues::PrimaryKey => {
                    options.push("PRIMARY KEY".to_string());
                }
                CreateColumnOptionsValues::Default(default) => {
                    options.push(format!("DEFAULT {default}"));
                }
                CreateColumnOptionsValues::Unique => {
                    options.push("UNIQUE".to_string());
                }
                CreateColumnOptionsValues::Check(check) => {
                    options.push(format!("CHECK({check})"));
                }
            }
        }

        options.join("  ")
    }

    fn setup_migration_table(&self) -> Result<(), DatabaseStrategyError> {
        let sql = "CREATE TABLE _migrations (
            table_name TEXT NOT NULL,
            last_migration INTEGER NOT NULL,
        )";

        self.conn
            .execute(sql, [])
            .map_err(|e| DatabaseStrategyError::MigrationTable(e.to_string()))?;

        Ok(())
    }

    fn on_migration_run(&self, table_name: &str, count: i64) -> Result<(), DatabaseStrategyError> {
        let sql = "INSERT INTO _migrations (table_name, last_migration) values (?1, ?2)";
        self.conn
            .execute(sql, params![table_name, count])
            .map_err(|e| DatabaseStrategyError::MigrationTable(e.to_string()))?;

        Ok(())
    }

    fn get_last_migration(&self, table_name: &str) -> Result<i64, DatabaseStrategyError> {
        let sql = "select * from _migrations where table_name = ?1";
        let mut result = self
            .conn
            .prepare(sql)
            .map_err(|e| DatabaseStrategyError::MigrationTable(e.to_string()))?;

        let result = result
            .query_map(params![table_name], |row| {
                row.get("last_migration").map(|e: i64| e)
            })
            .map_err(|e| DatabaseStrategyError::MigrationTable(e.to_string()))?;

        let result = result.into_iter().flatten().max().unwrap_or(0);

        Ok(result)
    }
}
