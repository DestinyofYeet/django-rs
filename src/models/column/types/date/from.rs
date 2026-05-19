use chrono::{DateTime, Utc};

use crate::models::column::ColumnValue;

impl From<DateTime<Utc>> for ColumnValue {
    fn from(value: DateTime<Utc>) -> Self {
        ColumnValue::Date(value)
    }
}
