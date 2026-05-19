use crate::models::column::ColumnValue;

impl From<i64> for ColumnValue {
    fn from(value: i64) -> Self {
        ColumnValue::Integer(value)
    }
}

impl From<f64> for ColumnValue {
    fn from(value: f64) -> Self {
        Self::Float(value)
    }
}
