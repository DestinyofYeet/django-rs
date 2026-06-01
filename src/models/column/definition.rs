use chrono::{DateTime, Utc};

#[derive(Debug, Clone, PartialEq)]
pub enum ColumnValue {
    String(String),
    Integer(i64),
    Float(f64),
    Date(DateTime<Utc>),
    Json(String),
    Null,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum ColumnType {
    String,
    Json,
    Integer,
    Float,
    Date,
    Bool,
}
