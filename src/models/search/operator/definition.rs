pub struct MaybeSearchOp(pub(crate) Option<SearchOp>);
impl MaybeSearchOp {
    pub fn get(self) -> Option<SearchOp> {
        self.0
    }
}

#[derive(Debug)]
pub enum SearchOp {
    EQ,
    NEQ,
    GT,
    GTEQ,
    LT,
    LTEQ,
}

#[allow(clippy::to_string_trait_impl)]
impl ToString for SearchOp {
    fn to_string(&self) -> String {
        match self {
            SearchOp::EQ => "=",
            SearchOp::NEQ => "!=",
            SearchOp::GT => ">",
            SearchOp::GTEQ => ">=",
            SearchOp::LT => "<",
            SearchOp::LTEQ => "<=",
        }
        .to_string()
    }
}
