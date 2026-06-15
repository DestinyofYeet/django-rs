use itertools::Itertools;

use crate::models::{
    MigrationKind, ModelMigration,
    column::{ColumnType, ModifyColumnOptionsValues},
};

pub trait Model {
    const TABLE_NAME: &'static str;

    /// This function should return the migration path for this Model
    fn get_migration() -> Vec<ModelMigration>;

    /// This function controls wether the model is saved or inserted into the database
    fn get_id(&self) -> Option<i64>;

    /// This function sets the id returned by the database
    fn set_id(&mut self, id: i64);

    /// This function returns the latest name of a column by traversing the migration path.
    /// An option of None indicates that the Column was dropped in the migration path
    fn get_latest_column_name(initial_name: &str) -> Option<String> {
        let mut past_names = vec![initial_name.to_string()];
        let mut name = Some(String::from(initial_name));

        for migration in Self::get_migration()
            .into_iter()
            .sorted_by_key(|item| item.ordering)
        {
            match migration.kind {
                MigrationKind::Create(_) => {}
                MigrationKind::Modify(modifiers) => {
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

    /// This function returns all columns and types defined by the get_migration()
    fn get_columns() -> Vec<(String, ColumnType)> {
        let migration = &Self::get_migration()[0];

        if let MigrationKind::Create(value) = &migration.kind {
            return value
                .iter()
                .map(|e| (Self::get_latest_column_name(&e.key).unwrap(), e.value))
                .collect_vec();
        }

        panic!("First migration is not a creation!");
    }

    /// This function is a helper intended for use in Box<dyn ...> situations where T is not available
    fn self_get_migration(&self) -> Vec<ModelMigration> {
        Self::get_migration()
    }

    /// This function is a helper intended for use in Box<dyn ...> situations where T is not available
    fn self_get_table_name(&self) -> &'static str {
        Self::TABLE_NAME
    }

    /// This function is a helper intended for use in Box<dyn ...> situations where T is not available
    fn self_get_columns(&self) -> Vec<(String, ColumnType)> {
        Self::get_columns()
    }
}
