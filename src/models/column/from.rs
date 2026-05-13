use crate::models::ColumnValue;

impl From<i64> for ColumnValue {
    fn from(value: i64) -> Self {
        ColumnValue::Integer(value)
    }
}

impl From<&str> for ColumnValue {
    fn from(value: &str) -> Self {
        Self::String(value.into())
    }
}

impl From<String> for ColumnValue {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<&String> for ColumnValue {
    fn from(value: &String) -> Self {
        Self::String(value.into())
    }
}

impl From<f64> for ColumnValue {
    fn from(value: f64) -> Self {
        Self::Float(value)
    }
}
