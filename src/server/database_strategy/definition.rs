use thiserror::Error;

use crate::models::{ColumnCreateOptions, Model, ModelValueType};

#[derive(Error, Debug)]
pub enum DatabaseStrategyError {
    #[error("Failed to migrate model: {0}")]
    MigrateModel(String),
}

pub trait DatabaseStrategy {
    fn table_exists(&self, table_name: &str) -> Result<bool, DatabaseStrategyError>;
    fn migrate_model<M: Model>(&self) -> Result<(), DatabaseStrategyError>;

    fn match_model(value: &ModelValueType) -> String;

    fn match_column_creation(value: &ColumnCreateOptions) -> String;
}
