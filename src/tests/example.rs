use crate::models::search::SearchQuery;
use crate::server::database_strategy::DatabaseStrategy;
use std::sync::LazyLock;

use django_rs_macro::{FromIter, SaveData};

use crate as django_rs;
use crate::models::column::{ColumnType, CreateColumn, CreateOptions};
use crate::models::{MigrationKind, ModelMigration};

use crate::{models::traits::model::Model, tests::setup_sqlite_server};

#[derive(FromIter, SaveData, Debug, PartialEq)]
pub struct MyStruct {
    id: Option<i64>,

    name: String,
    value: i32,
}

impl Model for MyStruct {
    const TABLE_NAME: &'static str = "MyStructs";

    fn get_migration() -> &'static Vec<crate::models::ModelMigration> {
        static MIGRATION: LazyLock<Vec<ModelMigration>> = LazyLock::new(|| {
            vec![ModelMigration::new(
                // This is the ordering. The framework will step through the Migrations in the sorted order.
                0,
                MigrationKind::Create(vec![
                    // A 'id' Column is currently required. Once I abstract get_migration in some way, this shouldn't be required
                    CreateColumn::new(
                        "id",
                        ColumnType::Integer,
                        CreateOptions::default().set_primary_key(),
                    ),
                    CreateColumn::new(
                        "name",
                        ColumnType::String,
                        CreateOptions::default().set_non_nullable().set_unique(),
                    ),
                    CreateColumn::new(
                        "value",
                        ColumnType::Integer,
                        CreateOptions::default().set_non_nullable(),
                    ),
                ]),
            )]
        });

        &MIGRATION
    }

    fn get_id(&self) -> Option<i64> {
        self.id
    }

    fn set_id(&mut self, id: i64) {
        self.id = Some(id);
    }

    fn get_id_column_name(&self) -> &'static str {
        "id"
    }
}

#[test]
pub fn readme_example() {
    let server = setup_sqlite_server();
    let db = server.get_database();
    db.migrate_model::<MyStruct>().unwrap();

    let mut my_struct = MyStruct {
        id: None,
        name: "some_name".to_string(),
        value: 1337,
    };

    db.save_model(&db.get_connection(), &mut my_struct).unwrap();

    let my_retrieved_struct: MyStruct = db
        .search_single_model::<MyStruct>(
            &db.get_connection(),
            SearchQuery::empty()
                // This searches for id = {my_struct.id}
                .add_constraint(("id", my_struct.id.unwrap())),
        )
        .unwrap()
        .unwrap();

    assert_eq!(my_retrieved_struct, my_struct);
}
