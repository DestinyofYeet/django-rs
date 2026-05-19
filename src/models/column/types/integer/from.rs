use crate::models::column::ColumnValue;

impl From<i64> for ColumnValue {
    fn from(value: i64) -> Self {
        ColumnValue::Integer(value)
    }
}
