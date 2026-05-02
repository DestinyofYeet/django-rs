use thiserror::Error;

use crate::models::{Model, ModelIteration};

#[derive(Error, Debug)]
pub enum DatabaseStrategyError {
    #[error("Failed to migrate model: {0}")]
    MigrateModel(String),
}

pub trait DatabaseStrategy {
    fn migrate_model(&self, model: Box<dyn Model>) -> Result<(), DatabaseStrategyError>;
}
