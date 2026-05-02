#[derive(Clone, Copy)]
pub enum ModelValueType {
    String,
    Integer,
    Float,
    Date,
}

#[derive(Debug, Default)]
pub struct ColumnCreateOptions {
    pub(crate) nullable: bool,
    pub(crate) primary_key: bool,
}

impl ColumnCreateOptions {
    pub fn set_nullable(mut self, value: bool) -> Self {
        self.nullable = value;

        self
    }

    pub fn set_primary_key(mut self, value: bool) -> Self {
        self.primary_key = value;

        self
    }
}

// pub enum ModelAction {
//     Create(ModelCreateOptions),
//     RenameField { from: String, to: String },
// }

pub struct ColumnCreation {
    pub(crate) key: String,
    pub(crate) value: ModelValueType,
    pub(crate) action: ColumnCreateOptions,
}

impl ColumnCreation {
    pub fn create_column(
        key: impl ToString,
        value: ModelValueType,
        action: ColumnCreateOptions,
    ) -> Self {
        Self {
            key: key.to_string(),
            value,
            action,
        }
    }
}

pub struct ModelIteration {
    pub(crate) data: Vec<ColumnCreation>,
}

impl ModelIteration {
    pub fn new(data: Vec<ColumnCreation>) -> Self {
        Self { data }
    }
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
