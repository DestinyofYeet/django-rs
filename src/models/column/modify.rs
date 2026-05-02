use std::collections::HashSet;

use crate::models::{ColumnType, column::CreateColumnOptions};

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum ModifyColumnOptionsValues {
    Rename {
        to: String,
    },
    Drop,
    Modify {
        new_type: ColumnType,
        new_options: CreateColumnOptions,
    },
}

pub struct ModifyColumnOptions {
    pub(crate) options: HashSet<ModifyColumnOptionsValues>,
}
