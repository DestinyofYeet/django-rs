use tracing::error;

use crate::models::column::ColumnType;
use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum CreateColumnOptionsValues {
    NonNullable,
    PrimaryKey,
    Default(String),
    Unique,
    Check(String),
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum CreateTableOptionValues {
    ForeignKey { table: String, column: String },
}

#[derive(Debug, Default, Eq, PartialEq)]
pub struct CreateOptions {
    pub(crate) column_options: HashSet<(u64, CreateColumnOptionsValues)>,
    pub(crate) table_options: HashSet<CreateTableOptionValues>,
}

impl CreateOptions {
    /// This column can be null (default: false)
    pub fn set_non_nullable(mut self) -> Self {
        self.column_options
            .insert((0, CreateColumnOptionsValues::NonNullable));

        self
    }

    /// This column should be a primary key (default: false)
    /// This implies `set_nullable()`
    /// The type of this column should be `Integer`
    pub fn set_primary_key(mut self) -> Self {
        self = self.set_non_nullable();
        self.column_options
            .insert((0, CreateColumnOptionsValues::PrimaryKey));

        self
    }

    /// This column should have a default value (default: None)
    pub fn set_default(mut self, value: String) -> Self {
        self.column_options
            .insert((0, CreateColumnOptionsValues::Default(value)));

        self
    }

    /// This column should only have unique values (default: false)
    pub fn set_unique(mut self) -> Self {
        self.column_options
            .insert((0, CreateColumnOptionsValues::Unique));

        self
    }

    /// This column should only have values that pass a check
    /// If you want to write a check like this
    /// ```sql
    /// CREATE TABLE test (
    ///   value INT CHECK(value > 0)
    /// )
    /// ```
    /// this needs to be called like `set_check("value > 0")`
    pub fn set_check(mut self, value: String) -> Self {
        self.column_options
            .insert((0, CreateColumnOptionsValues::Check(value)));

        self
    }

    /// This column will reference a foreign key
    pub fn set_foreign_key(mut self, table: impl ToString, column: impl ToString) -> Self {
        self.table_options
            .insert(CreateTableOptionValues::ForeignKey {
                table: table.to_string(),
                column: column.to_string(),
            });

        self
    }

    fn validate(&self, column_name: &str, column_type: ColumnType) -> bool {
        let mut is_ok = true;

        let mut mk_error = |msg: String| {
            is_ok = false;
            error!("Failed to validate create options for {column_name}: {msg}");
        };

        for (_, option) in self.column_options.iter() {
            match option {
                CreateColumnOptionsValues::NonNullable => {}
                CreateColumnOptionsValues::PrimaryKey => {
                    if column_type != ColumnType::Integer {
                        mk_error(format!(
                            "Column must be an integer (but it has type {column_type:?}) because it was selected as a primary key!",
                        ))
                    }
                }
                CreateColumnOptionsValues::Default(default) => match column_type {
                    ColumnType::String => {}
                    ColumnType::Integer => {
                        if let Err(e) = default.parse::<i64>() {
                            mk_error(format!(
                                "Default value '{default}' does not parse into a i64: {e}"
                            ))
                        }
                    }
                    ColumnType::Float => {
                        if let Err(e) = default.parse::<f64>() {
                            mk_error(format!(
                                "Default value '{default}' does not parse into a f64: {e}"
                            ))
                        }
                    }
                    ColumnType::Date => {}
                    ColumnType::Json => {}
                    ColumnType::Bool => {}
                },
                CreateColumnOptionsValues::Unique => {}
                CreateColumnOptionsValues::Check(_) => {}
            }
        }

        is_ok
    }
}

pub struct CreateColumn {
    pub(crate) key: String,
    pub(crate) value: ColumnType,
    pub(crate) options: CreateOptions,
}

impl CreateColumn {
    pub fn new(key: impl ToString, value: ColumnType, options: CreateOptions) -> Self {
        let key = key.to_string();

        if !options.validate(&key, value) {
            panic!("Failed to validate options!");
        }

        Self {
            key,
            value,
            options,
        }
    }
}
