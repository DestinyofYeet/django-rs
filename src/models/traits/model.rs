use itertools::Itertools;

use crate::models::{
    ModelIteration,
    column::{ColumnType, ModifyColumnOptionsValues},
    save::SaveModel,
};

pub trait Model {
    const TABLE_NAME: &'static str;

    fn get_migration() -> Vec<ModelIteration>;

    fn get_id(&self) -> Option<i64>;
    fn set_id(&mut self, id: i64);

    fn get_save_data(&self) -> Vec<SaveModel>;

    fn get_latest_column_name(initial_name: &str) -> Option<String> {
        let mut past_names = vec![initial_name.to_string()];
        let mut name = Some(String::from(initial_name));

        for migration in Self::get_migration() {
            match migration {
                ModelIteration::Create(_) => {}
                ModelIteration::Modify(modifiers) => {
                    for modification in modifiers {
                        if !past_names.contains(&modification.key) {
                            continue;
                        }

                        match modification.options {
                            ModifyColumnOptionsValues::Rename { to } => {
                                name = Some(to);
                                past_names.push(modification.key.clone());
                            }

                            ModifyColumnOptionsValues::Drop => name = None,
                            ModifyColumnOptionsValues::Add {
                                new_type: _,
                                new_options: _,
                            } => {}
                        }
                    }
                }
            }
        }

        name
    }

    fn get_columns() -> Vec<(String, ColumnType)> {
        let migration = &Self::get_migration()[0];
        if let ModelIteration::Create(value) = migration {
            return value
                .iter()
                .map(|e| (Self::get_latest_column_name(&e.key).unwrap(), e.value))
                .collect_vec();
        }

        panic!("First migration is not a creation!");
    }

    fn self_get_migration(&self) -> Vec<ModelIteration> {
        Self::get_migration()
    }

    fn self_get_table_name(&self) -> &'static str {
        Self::TABLE_NAME
    }
}
