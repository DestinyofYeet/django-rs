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

#[derive(Eq, Hash, PartialEq, Debug)]
pub enum SearchSelectOptions {
    Min,
    Max,
    Columns(Vec<String>),
}

#[derive(Debug)]
pub struct SearchQuery {
    pub(crate) constraints: Vec<SearchConstraint>,
    pub(crate) post_options: HashSet<SearchOptions>,
    pub(crate) select_options: HashSet<(u8, SearchSelectOptions)>,
}

impl SearchQuery {
    pub fn empty() -> Self {
        Self {
            constraints: Vec::new(),
            post_options: HashSet::new(),
            select_options: HashSet::new(),
        }
    }

    pub fn add_constraint(mut self, constraint: impl Into<SearchConstraint>) -> Self {
        self.constraints.push(constraint.into());
        self
    }

    pub fn set_limit(mut self, limit: i64) -> Self {
        self.post_options.insert(SearchOptions::Limit(limit));
        self
    }

    pub fn select_min(mut self) -> Self {
        self.select_options.insert((1, SearchSelectOptions::Min));
        self
    }

    pub fn select_max(mut self) -> Self {
        self.select_options.insert((1, SearchSelectOptions::Max));
        self
    }

    pub fn select_columns(mut self, columns: Vec<String>) -> Self {
        self.select_options
            .insert((0, SearchSelectOptions::Columns(columns)));
        self
    }
}
