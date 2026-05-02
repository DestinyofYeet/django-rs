#[derive(Clone, Copy)]
pub enum ModelValueType {
    String,
    Integer,
    Float,
    Date,
}

#[derive(Debug, Default)]
pub struct ModelCreateOptions {
    nullable: bool,
}

impl ModelCreateOptions {
    pub fn set_nullable(mut self, value: bool) -> Self {
        self.nullable = value;

        self
    }
}

pub enum ModelAction {
    Create(ModelCreateOptions),
    RenameField { from: String, to: String },
}

pub struct ModelFieldType {
    pub(crate) key: String,
    pub(crate) value: ModelValueType,
    pub(crate) action: ModelAction,
}

impl ModelFieldType {
    pub fn new(key: impl ToString, value: ModelValueType, action: ModelAction) -> Self {
        Self {
            key: key.to_string(),
            value,
            action,
        }
    }
}

pub struct ModelIteration {
    pub(crate) iteration: u64,
    pub(crate) model_name: String,
    pub(crate) data: Vec<ModelFieldType>,
}

impl ModelIteration {
    pub fn new(iteration: u64, model_name: impl ToString, data: Vec<ModelFieldType>) -> Self {
        Self {
            iteration,
            data,
            model_name: model_name.to_string(),
        }
    }
}

pub trait Model {
    fn get_fields() -> Vec<ModelIteration>;
}
