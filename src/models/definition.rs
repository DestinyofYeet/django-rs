use crate::models::column::{CreateColumn, ModifyColumn};

pub enum ModelIteration {
    Create(Vec<CreateColumn>),
    Modify(Vec<ModifyColumn>),
}
