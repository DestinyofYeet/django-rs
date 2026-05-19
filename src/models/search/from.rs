use crate::models::{
    column::ColumnValue,
    search::{SearchConstraint, operator::SearchOp},
};

impl<T> From<(&str, T)> for SearchConstraint
where
    T: Into<ColumnValue>,
{
    fn from(value: (&str, T)) -> Self {
        Self::new(value.0, SearchOp::EQ, (value.1).into())
    }
}

impl<T> From<(&str, SearchOp, T)> for SearchConstraint
where
    T: Into<ColumnValue>,
{
    fn from(value: (&str, SearchOp, T)) -> Self {
        Self::new(value.0, value.1, value.2.into())
    }
}
