use chrono::{DateTime, Utc};
use itertools::Itertools;

use crate::{
    models::{
        column::{CreateColumn, ModifyColumn, ModifyColumnOptionsValues},
        save::SaveModel,
    },
    server::database_strategy::DatabaseStrategyError,
};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum ColumnType {
    String,
    Integer,
    Float,
    Date,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ColumnValue {
    String(String),
    Integer(i64),
    Float(f64),
    Date(DateTime<Utc>),
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

    fn get_id(&self) -> Option<i64>;
    fn set_id(&mut self, id: i64);

    fn get_save_data(&self) -> Vec<SaveModel>;

    fn get_latest_column_name(initial_name: &str) -> Option<String> {
        let mut past_names = vec![initial_name.to_string()];
        let mut name = Some(String::from(initial_name));

        for migration in Self::get_migration().data {
            match migration {
                ModelIteration::Create(_) => {}
                ModelIteration::Modify(modifiers) => {
                    for modification in modifiers {
                        if !past_names.contains(&modification.key) {
                            continue;
                        }

                        match modification.options {
                            ModifyColumnOptionsValues::Rename { to } => {
                                name = Some(to);
                                past_names.push(modification.key.clone());
                            }

                            ModifyColumnOptionsValues::Drop => name = None,
                            ModifyColumnOptionsValues::Add {
                                new_type: _,
                                new_options: _,
                            } => {}
                        }
                    }
                }
            }
        }

        name
    }

    fn get_columns() -> Vec<(String, ColumnType)> {
        let migration = &Self::get_migration().data[0];
        if let ModelIteration::Create(value) = migration {
            return value
                .iter()
                .map(|e| (Self::get_latest_column_name(&e.key).unwrap(), e.value))
                .collect_vec();
        }

        panic!("First migration is not a creation!");
    }

    fn self_get_migration(&self) -> ModelMigration {
        Self::get_migration()
    }

    fn from_iter(iter: impl Iterator<Item = (String, String)>) -> Option<Self>
    where
        Self: Sized;
}
