use crate::models::definition::ColumnValue;

#[derive(Debug)]
pub struct SaveModel {
    pub(crate) key: String,
    pub(crate) value: Option<ColumnValue>,
}

impl SaveModel {
    pub fn new(key: impl ToString, value: Option<ColumnValue>) -> Self {
        Self {
            key: key.to_string(),
            value,
        }
    }
}
