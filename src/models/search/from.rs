use crate::models::{column::ColumnValue, search::SearchConstraint};

impl<T> From<(&str, T)> for SearchConstraint
where
    T: Into<ColumnValue>,
{
    fn from(value: (&str, T)) -> Self {
        Self::new(value.0, (value.1).into())
    }
}
