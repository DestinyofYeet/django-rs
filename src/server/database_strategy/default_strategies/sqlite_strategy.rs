use std::marker::PhantomData;

use itertools::Itertools;
use rusqlite::{Connection, Transaction, params};
use tracing::{debug, info, trace};

use crate::{
    models::{
        ColumnType, Model, ModelIteration,
        column::{CreateColumnOptions, CreateColumnOptionsValues, ModifyColumnOptionsValues},
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
    type ConnectionType<'a> = &'a Connection;

    fn get_connection(&self) -> Self::ConnectionType<'_> {
        &self.conn
    }

    fn migrate_model<M: Model>(&self) -> Result<(), DatabaseStrategyError> {
        let migration_data = M::get_migration();

        let table_name = &migration_data.model_name;

        if migration_data.data.is_empty() {
            return Err(DatabaseStrategyError::MigrateModel(format!(
                "Migration for model {} needs to have at least one migration!",
                table_name
            )));
        }

        let transaction =
            Transaction::new_unchecked(&self.conn, rusqlite::TransactionBehavior::Deferred)
                .map_err(|e| DatabaseStrategyError::Transaction(e.to_string()))?;

        for (count, migration) in migration_data.data.iter().enumerate() {
            self.setup_migration_table(self.get_connection())?;
            if let Some(migration) = self.get_last_migration(&transaction, table_name)?
                && migration >= count as i64
            {
                debug!(
                    "{}: migration {} not needed, idx {}",
                    table_name, migration, count,
                );
                continue;
            }

            match migration {
                ModelIteration::Create(columns) => {
                    if count != 0 {
                        return Err(DatabaseStrategyError::MigrateModel(format!(
                            "Can only create a table at the first iteration, not at iteration {count}"
                        )));
                    }

                    if self.table_exists(&transaction, table_name)? {
                        continue;
                    }

                    let mut sql = format!("CREATE TABLE {} (\n", table_name);
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

                    transaction
                        .execute(&sql, [])
                        .map_err(|e| DatabaseStrategyError::MigrateModel(e.to_string()))?;

                    trace!("produced sql: {sql}");
                    info!("Created table {}", migration_data.model_name);
                }
                ModelIteration::Modify(columns) => {
                    for col in columns {
                        let mut sql = format!("ALTER TABLE {table_name}\n");
                        sql += &SqliteStrategy::match_modify_column_options(&col.options, &col.key)
                            .to_string();

                        sql += ";";
                        trace!("produced sql: {sql}");

                        transaction
                            .execute(&sql, [])
                            .map_err(|e| DatabaseStrategyError::MigrateModel(e.to_string()))?;
                    }
                }
            }

            self.on_migration_run(&transaction, table_name, count as i64)?;
        }

        transaction
            .commit()
            .map_err(|e| DatabaseStrategyError::Transaction(e.to_string()))?;

        Ok(())
    }

    fn table_exists(
        &self,
        conn: &Connection,
        table_name: &str,
    ) -> Result<bool, DatabaseStrategyError> {
        conn.table_exists(None, table_name)
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

    fn setup_migration_table(&self, conn: &Connection) -> Result<(), DatabaseStrategyError> {
        if self.table_exists(conn, "_migrations")? {
            return Ok(());
        }

        let sql = "CREATE TABLE _migrations (
            table_name TEXT NOT NULL,
            last_migration INTEGER NOT NULL
        )";

        conn.execute(sql, [])
            .map_err(|e| DatabaseStrategyError::MigrationTable(e.to_string()))?;

        Ok(())
    }

    fn on_migration_run(
        &self,
        conn: &Connection,
        table_name: &str,
        count: i64,
    ) -> Result<(), DatabaseStrategyError> {
        let sql = "INSERT INTO _migrations (table_name, last_migration) values (?1, ?2)";
        conn.execute(sql, params![table_name, count])
            .map_err(|e| DatabaseStrategyError::MigrationTable(e.to_string()))?;

        Ok(())
    }

    fn get_last_migration(
        &self,
        conn: &Connection,
        table_name: &str,
    ) -> Result<Option<i64>, DatabaseStrategyError> {
        let sql = "select * from _migrations where table_name = ?1";
        let mut result = conn
            .prepare(sql)
            .map_err(|e| DatabaseStrategyError::MigrationTable(e.to_string()))?;

        let result = result
            .query_map(params![table_name], |row| {
                row.get("last_migration").map(|e: i64| e)
            })
            .map_err(|e| DatabaseStrategyError::MigrationTable(e.to_string()))?;

        let result = result.into_iter().flatten().max().map(Some).unwrap_or(None);

        Ok(result)
    }

    fn match_modify_column_options(
        value: &crate::models::column::ModifyColumnOptionsValues,
        column_name: &str,
    ) -> String {
        let mut out = String::new();
        match value {
            ModifyColumnOptionsValues::Rename { to } => {
                out.push_str(&format!("RENAME COLUMN {column_name} TO {to}"));
            }
            ModifyColumnOptionsValues::Drop => {
                out.push_str(&format!("DROP COLUMN {column_name}"));
            }
            ModifyColumnOptionsValues::Add {
                new_type,
                new_options,
            } => {
                out.push_str(&format!(
                    "ADD COLUMN {column_name} {} {}",
                    SqliteStrategy::match_column_type(new_type),
                    SqliteStrategy::match_create_column_options(new_options, column_name)
                ));
            }
        }

        out
    }

    fn save_model(
        &self,
        conn: Self::ConnectionType<'_>,
        model: impl Model,
    ) -> Result<(), DatabaseStrategyError> {
        todo!()
    }
}
