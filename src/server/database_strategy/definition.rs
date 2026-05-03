use thiserror::Error;

use crate::models::{
    ColumnType, Model,
    column::{CreateColumnOptions, ModifyColumnOptionsValues},
};

#[derive(Error, Debug)]
pub enum DatabaseStrategyError {
    #[error("Failed to migrate model: {0}")]
    MigrateModel(String),

    #[error("Failed to setup migration table: {0}")]
    MigrationTable(String),

    #[error("Failed to create transaction: {0}")]
    Transaction(String),

    #[error("Error: {0}")]
    Error(String),
}

pub trait DatabaseStrategy {
    type ConnectionType<'a>
    where
        Self: 'a;

    fn get_connection(&self) -> Self::ConnectionType<'_>;

    /// This function should tell if a table exists
    fn table_exists(
        &self,
        conn: Self::ConnectionType<'_>,
        table_name: &str,
    ) -> Result<bool, DatabaseStrategyError>;

    /// This function should migrate a Model to the database
    fn migrate_model<M: Model>(&self) -> Result<(), DatabaseStrategyError>;

    /// This function should convert the ColumnType to the appropriate Database type
    fn match_column_type(value: &ColumnType) -> String;

    /// This function should convert the CreateColumnOptions to the appropriate Database syntax.
    /// This function should return valid sql
    fn match_create_column_options(value: &CreateColumnOptions, column_name: &str) -> String;

    fn match_modify_column_options(value: &ModifyColumnOptionsValues, column_name: &str) -> String;

    /// This function should setup the migration table
    fn setup_migration_table(
        &self,
        conn: Self::ConnectionType<'_>,
    ) -> Result<(), DatabaseStrategyError>;

    /// This function should record that a migration has run
    fn on_migration_run(
        &self,
        conn: Self::ConnectionType<'_>,
        table_name: &str,
        index: i64,
    ) -> Result<(), DatabaseStrategyError>;

    /// This function should return the last run migration for the given table
    fn get_last_migration(
        &self,
        conn: Self::ConnectionType<'_>,
        table_name: &str,
    ) -> Result<Option<i64>, DatabaseStrategyError>;

    /// This function should save the model to the database
    fn save_model(
        &self,
        conn: Self::ConnectionType<'_>,
        model: impl Model,
    ) -> Result<(), DatabaseStrategyError>;
}
