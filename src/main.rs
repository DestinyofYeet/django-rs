use chrono::{Date, DateTime, Local, Utc};
use clap::Parser;
use django_rs::{
    models::{
        ColumnType, ColumnValue, Model, ModelIteration, ModelMigration,
        column::{CreateColumn, CreateColumnOptions},
        save::SaveModel,
        search::{SearchConstraint, SearchQuery},
    },
    server::{
        Server,
        database_strategy::{DatabaseStrategy, default_strategies::SqliteStrategy},
    },
    tasks::{
        logstrategy::{LogStrategyType, default_strategies::tracing_strategy::TracingStrategy},
        taskrunnable::TaskRunnable,
    },
};
use std::{str::FromStr, thread, time::Duration};
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

#[derive(Debug)]
pub struct User {
    id: Option<i64>,
    username: String,
    email: String,
    created: DateTime<Utc>,
}

impl Model for User {
    fn get_migration() -> ModelMigration {
        ModelMigration::new(
            "Users",
            vec![ModelIteration::Create(vec![
                CreateColumn::new(
                    "id",
                    ColumnType::Integer,
                    CreateColumnOptions::default().set_primary_key(),
                ),
                CreateColumn::new(
                    "username",
                    ColumnType::String,
                    CreateColumnOptions::default().set_non_nullable(),
                ),
                CreateColumn::new(
                    "email",
                    ColumnType::String,
                    CreateColumnOptions::default().set_non_nullable(),
                ),
                CreateColumn::new(
                    "created",
                    ColumnType::Date,
                    CreateColumnOptions::default().set_non_nullable(),
                ),
            ])],
        )
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
        ]
    }

    fn get_id(&self) -> Option<i64> {
        self.id
    }

    fn set_id(&mut self, id: i64) {
        self.id = Some(id);
    }

    fn from_iter(iter: impl Iterator<Item = (String, String)>) -> Option<Self>
    where
        Self: Sized,
    {
        let mut id: Option<i64> = None;
        let mut username: Option<String> = None;
        let mut email: Option<String> = None;
        let mut created: Option<DateTime<Utc>> = None;

        for (key, value) in iter {
            match value {
                String { .. } if matches!(Self::get_latest_column_name("id"), Some(id_col) if id_col == key) => {
                    id = value.parse::<i64>().ok()
                }

                String { .. } if matches!(Self::get_latest_column_name("email"), Some(email_col) if email_col == key) =>
                {
                    email = Some(value);
                }

                String { .. } if matches!(Self::get_latest_column_name("username"), Some(username_col) if username_col == key) =>
                {
                    username = Some(value);
                }

                String { .. } if matches!(Self::get_latest_column_name("created"), Some(created_col) if created_col == key) =>
                {
                    created = DateTime::from_str(&value).ok();
                }

                _ => {}
            }
        }

        if let Some(id) = id
            && let Some(username) = username
            && let Some(email) = email
            && let Some(created) = created
        {
            return Some(Self {
                id: Some(id),
                username,
                email,
                created,
            });
        }

        None
    }
}

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

    let mut user = User {
        id: None,
        username: "test".to_string(),
        email: "test@test.test".to_string(),
        created: Local::now().to_utc(),
    };

    server.get_database().migrate_model::<User>()?;
    let db = server.get_database();
    let conn = db.get_connection();
    db.save_model(conn, &mut user)?;
    let conn = db.get_connection();
    let mut user = db
        .search_single_model::<User>(
            conn,
            SearchQuery::empty()
                .add_constraint(SearchConstraint::new("id", ColumnValue::Integer(1))),
        )?
        .unwrap();

    user.username = "roflrofl".to_string();

    db.save_model(conn, &mut user)?;

    db.remove_model::<User>(
        conn,
        SearchQuery::empty().add_constraint(SearchConstraint::new(
            "username",
            ColumnValue::String("roflrofl".to_string()),
        )),
    )?;

    server.shutdown()?;

    Ok(())
}
