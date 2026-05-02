use thiserror::Error;

use crate::models::{ColumnType, Model, column::CreateColumnOptions};

#[derive(Error, Debug)]
pub enum DatabaseStrategyError {
    #[error("Failed to migrate model: {0}")]
    MigrateModel(String),

    #[error("Failed to setup migration table: {0}")]
    MigrationTable(String),
}

pub trait DatabaseStrategy {
    /// This function should tell if a table exists
    fn table_exists(&self, table_name: &str) -> Result<bool, DatabaseStrategyError>;

    /// This function should migrate a Model to the database
    fn migrate_model<M: Model>(&self) -> Result<(), DatabaseStrategyError>;

    /// This function should convert the ColumnType to the appropriate Database type
    fn match_column_type(value: &ColumnType) -> String;

    /// This function should convert the CreateColumnOptions to the appropriate Database syntax.
    /// This function should return valid sql
    fn match_create_column_options(value: &CreateColumnOptions, column_name: &str) -> String;

    /// This function should setup the migration table
    fn setup_migration_table(&self) -> Result<(), DatabaseStrategyError>;

    /// This function should record that a migration has run
    fn on_migration_run(&self, table_name: &str, index: i64) -> Result<(), DatabaseStrategyError>;

    /// This function should return the last run migration for the given table
    fn get_last_migration(&self, table_name: &str) -> Result<i64, DatabaseStrategyError>;
}
