use crate::models::column::ColumnValue;

impl From<f64> for ColumnValue {
    fn from(value: f64) -> Self {
        ColumnValue::Float(value)
    }
}
