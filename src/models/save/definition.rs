use crate::models::column::ColumnValue;

#[derive(Debug)]
pub struct SaveModel {
    pub(crate) key: String,
    pub(crate) value: Option<ColumnValue>,
}

impl SaveModel {
    pub fn new<T>(key: impl ToString, value: Option<T>) -> Self
    where
        T: Into<ColumnValue>,
    {
        Self {
            key: key.to_string(),
            value: value.map(|e| e.into()),
        }
    }
}
