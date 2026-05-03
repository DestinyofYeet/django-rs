use crate::models::column::{CreateColumn, ModifyColumn};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum ColumnType {
    String,
    Integer,
    Float,
    Date,
}

pub enum ModelIteration {
    Create(Vec<CreateColumn>),
    Modify(Vec<ModifyColumn>),
}

pub struct ModelMigration {
    pub(crate) model_name: String,
    pub(crate) data: Vec<ModelIteration>,
}

impl ModelMigration {
    pub fn new(model_name: impl ToString, data: Vec<ModelIteration>) -> Self {
        Self {
            model_name: model_name.to_string(),
            data,
        }
    }
}

pub trait Model {
    fn get_migration() -> ModelMigration;
}
