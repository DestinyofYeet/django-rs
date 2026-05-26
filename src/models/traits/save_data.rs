use itertools::Itertools;

use crate::models::{column::ColumnType, save::SaveModel, traits::model::Model};

pub trait SaveData {
    /// This function should return all columns and values which were defined by get_migration()
    fn get_save_data(&self) -> Vec<SaveModel>;
}

pub trait ValidateSaveData {
    /// This function checks if all columns are used in the save_data
    fn validate_save_data(&self) -> Option<Vec<String>>;
}

impl<T> ValidateSaveData for T
where
    T: SaveData + Model,
{
    /// This function checks if all columns are used in the save_data
    fn validate_save_data(&self) -> Option<Vec<String>> {
        let cols = Self::get_columns();

        if !cols.contains(&("id".to_string(), ColumnType::Integer)) {
            return Some(["id (Integer)"].iter().map(|e| e.to_string()).collect_vec());
        }

        let save_data = self.get_save_data();

        let mut missing_save_data = Vec::new();

        for (name, c_type) in cols {
            if !save_data.iter().any(|model| model.key == name) {
                missing_save_data.push(format!("{name} {c_type:?}"));
            }
        }

        if missing_save_data.is_empty() {
            None
        } else {
            Some(missing_save_data)
        }
    }
}
