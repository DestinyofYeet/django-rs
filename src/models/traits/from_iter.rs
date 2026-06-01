use crate::models::column::ColumnType;

pub struct FromIterValue {
    pub column_name: String,
    pub column_value: String,
    pub column_type: ColumnType,
}

pub trait FromIter {
    fn from_iter(iter: impl Iterator<Item = FromIterValue>) -> Option<Self>
    where
        Self: Sized;
}
