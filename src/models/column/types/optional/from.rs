use crate::models::column::ColumnValue;

impl<T> From<Option<T>> for ColumnValue
where
    T: Into<ColumnValue>,
{
    fn from(value: Option<T>) -> Self {
        match value {
            Some(value) => value.into(),
            None => Self::Null,
        }
    }
}
