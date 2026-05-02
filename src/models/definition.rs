use std::collections::HashSet;

use tracing::Instrument;

#[derive(Clone, Copy)]
pub enum ColumnType {
    String,
    Integer,
    Float,
    Date,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum CreateColumnOptionsValues {
    Nullable,
    PrimaryKey,
    Default(String),
    Unique,
    Check(String),
}

#[derive(Debug, Default)]
pub struct CreateColumnOptions {
    pub(crate) options: HashSet<CreateColumnOptionsValues>,
}

impl CreateColumnOptions {
    /// This column can be null (default: false)
    pub fn set_nullable(mut self) -> Self {
        self.options.insert(CreateColumnOptionsValues::Nullable);

        self
    }

    /// This column should be a primary key (default: false)
    /// This implies `set_nullable()`
    /// The type of this column should be `Integer`
    pub fn set_primary_key(mut self) -> Self {
        self.options.insert(CreateColumnOptionsValues::Nullable);
        self.options.insert(CreateColumnOptionsValues::PrimaryKey);

        self
    }

    /// This column should have a default value (default: None)
    pub fn set_default(mut self, value: String) -> Self {
        self.options
            .insert(CreateColumnOptionsValues::Default(value));

        self
    }

    /// This column should only have unique values (default: false)
    pub fn set_unique(mut self) -> Self {
        self.options.insert(CreateColumnOptionsValues::Unique);

        self
    }

    /// This column should only have values that pass a check
    /// If you want to write a check like this
    /// ```sql
    /// CREATE TABLE test (
    ///   value INT CHECK(value > 0)
    /// )
    /// ```
    /// this needs to be called like `set_check("value > 0")`
    pub fn set_check(mut self, value: String) -> Self {
        self.options.insert(CreateColumnOptionsValues::Check(value));

        self
    }
}

pub struct CreateColumn {
    pub(crate) key: String,
    pub(crate) value: ColumnType,
    pub(crate) options: CreateColumnOptions,
}

impl CreateColumn {
    pub fn new(key: impl ToString, value: ColumnType, options: CreateColumnOptions) -> Self {
        Self {
            key: key.to_string(),
            value,
            options,
        }
    }
}

pub enum ModelIteration {
    Create(Vec<CreateColumn>),
    Modify,
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
