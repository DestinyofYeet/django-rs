use chrono::{DateTime, Local, Utc};
use clap::Parser;
use django_rs::{
    models::{
        ModelIteration,
        column::{ColumnType, ColumnValue, CreateColumn, CreateOptions},
        save::SaveModel,
        search::SearchQuery,
        traits::model::Model,
    },
    server::{
        Server,
        database_strategy::{
            DatabaseStrategy, TransactionOptions, default_strategies::SqliteStrategy,
        },
        database_tasks::SaveModelTask,
    },
    tasks::{
        logstrategy::{LogStrategyType, default_strategies::tracing_strategy::TracingStrategy},
        task::TaskState,
        taskrunnable::TaskRunnable,
    },
};
use django_rs_macro::FromIter;
use std::{thread, time::Duration};
use tracing_subscriber::EnvFilter;

#[derive(Parser)]
pub struct Args {
    #[arg(short='v', long, action = clap::ArgAction::Count, help="Sets the verbose level. More v's more output")]
    verbose: u8,
}

pub struct PrintTask {}

impl PrintTask {
    pub fn new() -> Box<Self> {
        Box::new(Self {})
    }
}

impl TaskRunnable for PrintTask {
    fn run(&mut self, logger: LogStrategyType, worker_id: u64) {
        thread::sleep(Duration::from_millis(300));
        logger.info(worker_id, "print");
    }
}

#[derive(Debug, FromIter)]
pub struct Group {
    id: Option<i64>,
    name: String,
}

impl Model for Group {
    const TABLE_NAME: &'static str = "groups";

    fn get_migration() -> Vec<ModelIteration> {
        vec![ModelIteration::Create(vec![
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
        ])]
    }

    fn get_id(&self) -> Option<i64> {
        self.id
    }

    fn set_id(&mut self, id: i64) {
        self.id = Some(id);
    }

    fn get_save_data(&self) -> Vec<SaveModel> {
        vec![
            SaveModel::new(
                Self::get_latest_column_name("id").unwrap(),
                self.id.map(ColumnValue::Integer),
            ),
            SaveModel::new(
                Self::get_latest_column_name("name").unwrap(),
                Some(self.name.as_str()),
            ),
        ]
    }
}

#[derive(Debug, FromIter)]
pub struct User {
    id: Option<i64>,
    username: String,
    email: String,
    created: DateTime<Utc>,
    group_id: i64,
}

impl Model for User {
    const TABLE_NAME: &'static str = "Users";

    fn get_migration() -> Vec<ModelIteration> {
        vec![ModelIteration::Create(vec![
            CreateColumn::new(
                "id",
                ColumnType::Integer,
                CreateOptions::default().set_primary_key(),
            ),
            CreateColumn::new(
                "username",
                ColumnType::String,
                CreateOptions::default().set_non_nullable(),
            ),
            CreateColumn::new(
                "email",
                ColumnType::String,
                CreateOptions::default().set_non_nullable(),
            ),
            CreateColumn::new(
                "created",
                ColumnType::Date,
                CreateOptions::default().set_non_nullable(),
            ),
            CreateColumn::new(
                "group_id",
                ColumnType::Integer,
                CreateOptions::default()
                    // .set_non_nullable()
                    .set_foreign_key("groups", "id"),
            ),
        ])]
    }

    fn get_save_data(&self) -> Vec<SaveModel> {
        vec![
            SaveModel::new(
                Self::get_latest_column_name("username").unwrap(),
                Some(ColumnValue::String(self.username.clone())),
            ),
            SaveModel::new(
                Self::get_latest_column_name("email").unwrap(),
                Some(ColumnValue::String(self.email.clone())),
            ),
            SaveModel::new(
                Self::get_latest_column_name("id").unwrap(),
                self.id.map(ColumnValue::Integer),
            ),
            SaveModel::new(
                Self::get_latest_column_name("created").unwrap(),
                Some(ColumnValue::Date(self.created)),
            ),
            SaveModel::new(
                Self::get_latest_column_name("group_id").unwrap(),
                Some(ColumnValue::Integer(self.group_id)),
            ),
        ]
    }

    fn get_id(&self) -> Option<i64> {
        self.id
    }

    fn set_id(&mut self, id: i64) {
        self.id = Some(id);
    }
}

// impl FromIter for User {
//     fn from_iter(iter: impl Iterator<Item = (String, String)>) -> Option<Self>
//     where
//         Self: Sized,
//     {
//         let mut id: Option<i64> = None;
//         let mut username: Option<String> = None;
//         let mut email: Option<String> = None;
//         let mut created: Option<DateTime<Utc>> = None;
//         let mut group_id: Option<i64> = None;

//         for (key, value) in iter {
//             match value {
//                 String { .. } if matches!(Self::get_latest_column_name("id"), Some(id_col) if id_col == key) => {
//                     id = value.parse::<i64>().ok()
//                 }

//                 String { .. } if matches!(Self::get_latest_column_name("email"), Some(email_col) if email_col == key) =>
//                 {
//                     email = Some(value);
//                 }

//                 String { .. } if matches!(Self::get_latest_column_name("username"), Some(username_col) if username_col == key) =>
//                 {
//                     username = Some(value);
//                 }

//                 String { .. } if matches!(Self::get_latest_column_name("created"), Some(created_col) if created_col == key) =>
//                 {
//                     created = DateTime::from_str(&value).ok();
//                 }

//                 String { .. } if matches!(Self::get_latest_column_name("group_id"), Some(group_col) if group_col == key ) =>
//                 {
//                     group_id = value.parse().ok();
//                 }

//                 _ => {}
//             }
//         }

//         if let Some(id) = id
//             && let Some(username) = username
//             && let Some(email) = email
//             && let Some(created) = created
//             && let Some(group_id) = group_id
//         {
//             return Some(Self {
//                 id: Some(id),
//                 username,
//                 email,
//                 created,
//                 group_id,
//             });
//         }

//         None
//     }
// }

fn main() -> Result<(), anyhow::Error> {
    let args = Args::parse();

    let level = match args.verbose {
        0 => "warn",
        1 => "info",
        2 => "debug",
        _ => "trace",
    };

    tracing_subscriber::fmt()
        .with_line_number(true)
        .with_env_filter(EnvFilter::new(level))
        .init();

    let server = Server::new(8, TracingStrategy {}, SqliteStrategy::new("tmp/db.sqlite"))?;

    let mut group = Group {
        id: None,
        name: "Test".to_string(),
    };

    server.get_database().migrate_model::<Group>()?;
    server.get_database().migrate_model::<User>()?;
    let db = server.get_database();
    let conn = db.get_connection();

    if let Some(found_group) = db.search_single_model::<Group>(
        &conn,
        SearchQuery::empty().add_constraint(("name", &group.name)),
    )? {
        group = found_group;
    } else {
        db.save_model(&conn, &mut group)?;
    };

    let mut user = User {
        id: None,
        username: "test".to_string(),
        email: "test@test.test".to_string(),
        created: Local::now().to_utc(),
        group_id: group.id.unwrap(),
    };

    db.save_model(&conn, &mut user)?;
    let conn = db.get_connection();
    let mut user = db
        .search_single_model::<User>(
            &conn,
            SearchQuery::empty().add_constraint(("id", ColumnValue::Integer(1))),
        )?
        .unwrap();

    user.username = "roflrofl".to_string();

    user.group_id = 5;

    let save_task = SaveModelTask::new(db.clone(), user);

    let task_handler = server.get_task_handler();
    task_handler.spawn_task(PrintTask::new());

    let task = task_handler.spawn_task(save_task);
    db.remove_model::<User>(
        &conn,
        &SearchQuery::empty().add_constraint(("username", "roflrofl")),
    )?;

    drop(conn);

    test(&server);

    server.shutdown()?;

    Ok(())
}

fn test<D>(server: &Server<D>)
where
    D: DatabaseStrategy,
{
    let db = server.get_database();
    db.with_transaction(|tx| {
        db.table_exists(&*tx, "hi").unwrap();
        db.manage_transaction(tx, TransactionOptions::Commit)
            .unwrap();
    })
    .unwrap();

    // let tx = db.get_transaction();

    // db.table_exists(&tx, "hi").unwrap();
    // db.manage_transaction(tx, TransactionOptions::Commit)
    // .unwrap();
}
