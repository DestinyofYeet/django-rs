use crate::models::column::{ColumnType, CreateOptions};

#[derive(Debug)]
pub enum ModifyColumnOptionsValues {
    Rename {
        to: String,
    },
    Drop,
    Add {
        new_type: ColumnType,
        new_options: CreateOptions,
    },
}

pub struct ModifyColumn {
    pub(crate) key: String,
    pub(crate) options: ModifyColumnOptionsValues,
}

impl ModifyColumn {
    pub fn new(key: impl ToString, options: ModifyColumnOptionsValues) -> Self {
        Self {
            key: key.to_string(),
            options,
        }
    }
}
