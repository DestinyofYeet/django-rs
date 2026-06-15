use std::collections::HashSet;

use itertools::Itertools;

use crate::models::{column::ColumnValue, search::operator::SearchOp};

#[derive(Debug, Clone)]
pub struct SearchConstraint {
    pub(crate) column: String,
    pub(crate) operator: SearchOp,
    pub(crate) value: ColumnValue,
}

impl SearchConstraint {
    pub fn new(column: impl ToString, operator: SearchOp, value: ColumnValue) -> Self {
        Self {
            operator,
            column: column.to_string(),
            value,
        }
    }
}

#[derive(Eq, Hash, PartialEq, Debug, Clone)]
pub enum SearchOptions {
    Limit(i64),
    OrderBy(Vec<(String, Option<SearchOrderByOptions>)>),
}

#[derive(Eq, Hash, PartialEq, Debug, Clone)]
pub enum SearchOrderByOptions {
    Asc,
    Desc,
}

#[derive(Eq, Hash, PartialEq, Debug, Clone)]
pub enum SearchSelectOptions {
    Min,
    Max,
    Columns(Vec<String>),
}

#[derive(Debug, Clone)]
pub struct SearchQuery {
    pub(crate) constraints: Vec<SearchConstraint>,
    pub(crate) post_options: HashSet<(u8, SearchOptions)>,
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
        self.post_options.insert((10, SearchOptions::Limit(limit)));
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

    pub fn add_order_by(
        mut self,
        order_by: Vec<(impl ToString, Option<SearchOrderByOptions>)>,
    ) -> Self {
        self.post_options.insert((
            0,
            SearchOptions::OrderBy(
                order_by
                    .into_iter()
                    .map(|(key, option)| (key.to_string(), option))
                    .collect_vec(),
            ),
        ));
        self
    }
}
