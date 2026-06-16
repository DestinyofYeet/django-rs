# Django-rs

I wanted to build something like the python framework Django

# How to use?

Take this struct for example

```rust

pub struct MyStruct {
  name: String,
  value: i32
}
```

To use this in django-rs, you need to derive and implement a few things.

```rust

#[derive(FromIter, SaveData)]
pub struct MyStruct {
  id: Option<i64>,

  name: String,
  value: i32
}

impl Model for MyStruct {
  // ...
}
  
```
A new id field has appeared. This id field controls wether the struct should be inserted or updated in the database. When creating a new instance of `MyStruct` set it to NULL. All `MyStruct`s from the Database have the id field set.

```rust
impl Model for MyStruct {
    // This is the name of the table that gets created
    const TABLE_NAME: &'static str = "MyStructs";

    // This is the migration path of the Model
    fn get_migration() -> &'static Vec<ModelMigration> {
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
                      CreateOptions::default().set_non_nullable()
                    )
                ]),
            )]
        });

        &MIGRATION
    }

    // This is the real check if it gets inserted or updated
    fn get_id(&self) -> Option<i64> {
        self.id
    }

    fn set_id(&mut self, id: i64) {
        self.id = Some(id);
    }
}
```

In your main function you can then initialise the server.
In the future I want to implement a `PostgresStrategy` and some other LoggingStrategy.

```rust
pub fn main() {
  let server = DjangoServer::new(8, TracingStrategy {}, SqliteStrategy::new("somePath.db"))?;
  let db = server.get_database()

  db.migrate_model::<MyStruct>().unwrap();

  let mut my_struct = MyStruct {
    id: None,
    name: "some_name".to_string(),
    value: 1337
  };

  // This will set the 'id' field
  db.save_model(&db.get_connection(), &mut my_struct).unwrap();

  let my_retrieved_struct: MyStruct = db.search_single_model::<MyStruct>(
      &db.get_connection(),
      SearchQuery::empty()
        // This searches for id = {my_struct.id}
        .add_constraint(("id", my_struct.id.unwrap()))
  )
  // this returns a Result<Option<MyStruct>, DatabaseStrategyError>
  .unwrap()
  .unwrap();

  assert_eq!(my_retrieved_struct, my_struct);
}

```
