#[derive(Clone, Copy)]
pub enum ColumnType {
    String,
    Integer,
    Float,
    Date,
}

#[derive(Debug, Default)]
pub struct CreateColumnOptions {
    pub(crate) nullable: bool,
    pub(crate) primary_key: bool,
}

impl CreateColumnOptions {
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
