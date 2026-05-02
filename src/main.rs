use clap::Parser;
use django_rs::{
    models::{
        Model, ModelAction, ModelCreateOptions, ModelFieldType, ModelIteration, ModelValueType,
    },
    server::{Server, database_strategy::default_strategies::SqliteStrategy},
    tasks::{
        logstrategy::{LogStrategyType, default_strategies::tracing_strategy::TracingStrategy},
        taskhandler::TaskHandler,
        taskrunnable::TaskRunnable,
    },
};
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

pub struct User {
    username: String,
    email: String,
}

impl Model for User {
    fn get_fields() -> Vec<ModelIteration> {
        vec![ModelIteration::new(
            0,
            "User",
            vec![
                ModelFieldType::new(
                    "username",
                    ModelValueType::String,
                    ModelAction::Create(ModelCreateOptions::default().set_nullable(false)),
                ),
                ModelFieldType::new(
                    "email",
                    ModelValueType::String,
                    ModelAction::Create(ModelCreateOptions::default().set_nullable(false)),
                ),
            ],
        )]
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

    let server = Server::new(8, TracingStrategy {}, SqliteStrategy::new(":memory:"))?;

    server.get_database().migrate_model()

    server.shutdown()?;

    Ok(())
}
