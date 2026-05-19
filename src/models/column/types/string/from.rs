use crate::models::column::ColumnValue;

impl From<String> for ColumnValue {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<&str> for ColumnValue {
    fn from(value: &str) -> Self {
        Self::String(value.to_string())
    }
}

impl From<&String> for ColumnValue {
    fn from(value: &String) -> Self {
        Self::String(value.clone())
    }
}
