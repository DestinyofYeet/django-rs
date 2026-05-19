use crate::models::search::operator::{MaybeSearchOp, SearchOp};

impl From<&str> for MaybeSearchOp {
    fn from(value: &str) -> Self {
        MaybeSearchOp(SearchOp::parse(value))
    }
}

impl SearchOp {
    pub fn parse(input: &str) -> Option<SearchOp> {
        Some(match input {
            "=" => SearchOp::EQ,
            "!=" => SearchOp::NEQ,
            "<" => SearchOp::LT,
            "<=" => SearchOp::LTEQ,
            ">" => SearchOp::GT,
            ">=" => SearchOp::GTEQ,

            _ => return None,
        })
    }
}
