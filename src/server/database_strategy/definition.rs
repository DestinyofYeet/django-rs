use thiserror::Error;

use crate::models::{ColumnType, CreateColumnOptions, Model};

#[derive(Error, Debug)]
pub enum DatabaseStrategyError {
    #[error("Failed to migrate model: {0}")]
    MigrateModel(String),
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
    fn match_create_column_options(value: &CreateColumnOptions) -> String;
}
