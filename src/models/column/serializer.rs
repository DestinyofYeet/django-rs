use std::{any::type_name, str::FromStr};

use serde::{Deserialize, Serialize};
use serde_json::{Number, Value};
use thiserror::Error;

use crate::models::column::{ColumnType, ColumnValue};

#[derive(Error, Debug, Deserialize)]
pub enum SerdeColumnError {
    #[error("Failed to serialize: {0}")]
    Serialize(String),

    #[error("Failed to deserialize: {0}")]
    Deserialize(String),
}

impl From<serde_json::Error> for SerdeColumnError {
    fn from(value: serde_json::Error) -> Self {
        Self::Serialize(value.to_string())
    }
}

pub trait ToColumn {
    fn to_column(&self) -> Result<ColumnValue, SerdeColumnError>;
}

impl<T> ToColumn for T
where
    T: Serialize,
{
    fn to_column(&self) -> Result<ColumnValue, SerdeColumnError> {
        let value = serde_json::to_value(self)?;

        Ok(match &value {
            serde_json::Value::Null => ColumnValue::Null,
            serde_json::Value::Bool(value) => {
                let value = if *value { 1 } else { 0 };

                ColumnValue::Integer(value)
            }
            serde_json::Value::Number(number) => {
                if number.to_string().contains(".") {
                    ColumnValue::Float(number.as_f64().unwrap())
                } else {
                    ColumnValue::Integer(number.as_i64().unwrap())
                }
            }
            serde_json::Value::String(string) => ColumnValue::String(string.clone()),
            serde_json::Value::Array(_) => ColumnValue::Json(value.to_string()),
            serde_json::Value::Object(_) => ColumnValue::Json(value.to_string()),
        })
    }
}

#[allow(clippy::wrong_self_convention)]
pub trait FromColumn<S> {
    fn from_column<T>(&self, column_type: ColumnType) -> Result<T, SerdeColumnError>
    where
        T: for<'a> Deserialize<'a> + std::fmt::Debug;
}

impl<S> FromColumn<S> for S
where
    Self: ToString,
{
    fn from_column<T>(&self, column_type: ColumnType) -> Result<T, SerdeColumnError>
    where
        T: for<'a> Deserialize<'a> + std::fmt::Debug,
    {
        let value = match column_type {
            ColumnType::String => Value::String(self.to_string()),

            ColumnType::Json => serde_json::from_str::<Value>(&self.to_string())?,

            ColumnType::Integer => {
                let int: i64 = self
                    .to_string()
                    .parse()
                    .map_err(|e: std::num::ParseIntError| {
                        SerdeColumnError::Deserialize(e.to_string())
                    })?;
                Value::Number(int.into())
            }
            ColumnType::Float => {
                let float: f64 =
                    self.to_string()
                        .parse()
                        .map_err(|e: std::num::ParseFloatError| {
                            SerdeColumnError::Deserialize(e.to_string())
                        })?;
                Value::Number(Number::from_f64(float).unwrap())
            }
            ColumnType::Date => Value::String(self.to_string()),
            ColumnType::Bool => {
                let value = self.to_string() == "1";

                Value::Bool(value)
            }
        };

        let parsed = serde_json::from_value::<T>(value);

        Ok(parsed?)
    }
}
