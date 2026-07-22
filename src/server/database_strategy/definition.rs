use crate::models::column::ColumnType;
use crate::models::column::ColumnValue;
use crate::models::traits::from_iter::FromIter;
use crate::models::traits::model::Model;
use crate::models::traits::save_data::SaveData;
use crate::models::traits::save_data::ValidateSaveData;
use std::{collections::HashSet, ops::Deref};

use thiserror::Error;

use crate::models::{
    column::{CreateOptions, CreateTableOptionValues, ModifyColumnOptionsValues},
    search::SearchQuery,
};

pub enum TransactionOptions {
    Commit,
    Rollback,
}

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

    #[error("Failed to save Model <{}>: {}", .model, .err)]
    SaveModel { err: String, model: &'static str },

    #[error("Failed to search Model: {0}")]
    SearchModel(String),

    #[error("Failed to delete Model: {0}")]
    DeleteModel(String),
}

pub trait DatabaseStrategy: Send + Sync {
    type ConnectionType<'a>: Sized + Deref<Target = Self::FunctionConnType<'a>>
    where
        Self: 'a;

    type FunctionConnType<'a>: Sized
    where
        Self: 'a;

    type TransactionType<'a>: Deref<Target = Self::FunctionConnType<'a>>
    where
        Self: 'a;

    /// This function should return a simple connection type.
    fn get_connection(&self) -> Self::ConnectionType<'_>;

    fn with_transaction<F, T>(&self, function: F) -> Result<T, DatabaseStrategyError>
    where
        F: FnOnce(Self::TransactionType<'_>) -> T;

    /// This function should tell if a table exists.
    fn table_exists(
        &self,
        conn: &Self::FunctionConnType<'_>,
        table_name: &str,
    ) -> Result<bool, DatabaseStrategyError>;

    /// This function should migrate a Model to the database.
    fn migrate_model<M: Model>(&self) -> Result<(), DatabaseStrategyError>;

    /// This function should setup the migration table.
    fn setup_migration_table(
        &self,
        conn: &Self::FunctionConnType<'_>,
    ) -> Result<(), DatabaseStrategyError>;

    /// This function should record that a migration has run.
    fn on_migration_run(
        &self,
        conn: &Self::FunctionConnType<'_>,
        table_name: &str,
        index: i64,
    ) -> Result<(), DatabaseStrategyError>;

    /// This function should return the last run migration for the given table.
    fn get_last_migration(
        &self,
        conn: &Self::FunctionConnType<'_>,
        table_name: &str,
    ) -> Result<Option<i64>, DatabaseStrategyError>;

    /// This function should save the model to the database.
    fn save_model<T>(
        &self,
        conn: &Self::FunctionConnType<'_>,
        model: &mut T,
    ) -> Result<(), DatabaseStrategyError>
    where
        T: SaveData + ValidateSaveData + Model + FromIter;

    /// This function will search for a singular model.
    fn search_single_model<T>(
        &self,
        conn: &Self::FunctionConnType<'_>,
        query: SearchQuery,
    ) -> Result<Option<T>, DatabaseStrategyError>
    where
        T: Model + FromIter;

    /// This function will search for multiple models.
    fn search_multiple_model<T>(
        &self,
        conn: &Self::FunctionConnType<'_>,
        query: SearchQuery,
    ) -> Result<Vec<T>, DatabaseStrategyError>
    where
        T: Model + FromIter;

    /// This function should remove a model from the database.
    fn remove_model<T: Model>(
        &self,
        conn: &Self::FunctionConnType<'_>,
        query: &SearchQuery,
    ) -> Result<(), DatabaseStrategyError>;

    /// This function should manage a transaction connection type.
    fn manage_transaction(
        &self,
        conn: Self::TransactionType<'_>,
        options: TransactionOptions,
    ) -> Result<(), DatabaseStrategyError>;
}
