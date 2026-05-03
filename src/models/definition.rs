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

pub enum SaveModelType {
    String(String),
    Integer(i64),
    Float(f64),
}

pub struct SaveModel {
    key: String,
    value: SaveModelType,
}

impl SaveModel {
    pub fn new(key: impl ToString, value: SaveModelType) -> Self {
        Self {
            key: key.to_string(),
            value,
        }
    }
}

pub trait Model {
    fn get_migration() -> ModelMigration;

    fn get_save_data(&self) -> Vec<SaveModel>;
}
