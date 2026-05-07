use std::collections::HashSet;

use crate::models::definition::ColumnValue;

#[derive(Debug)]
pub struct SearchConstraint {
    pub(crate) column: String,
    pub(crate) value: ColumnValue,
}

impl SearchConstraint {
    pub fn new(column: impl ToString, value: ColumnValue) -> Self {
        Self {
            column: column.to_string(),
            value,
        }
    }
}

#[derive(Eq, Hash, PartialEq, Debug)]
pub enum SearchOptions {
    Limit(i64),
}

#[derive(Debug)]
pub struct SearchQuery {
    pub(crate) constraints: Vec<SearchConstraint>,
    pub(crate) options: HashSet<SearchOptions>,
}

impl SearchQuery {
    pub fn empty() -> Self {
        Self {
            constraints: Vec::new(),
            options: HashSet::new(),
        }
    }

    pub fn add_constraint(mut self, constraint: SearchConstraint) -> Self {
        self.constraints.push(constraint);
        self
    }

    pub fn set_limit(mut self, limit: i64) -> Self {
        self.options.insert(SearchOptions::Limit(limit));
        self
    }
}
