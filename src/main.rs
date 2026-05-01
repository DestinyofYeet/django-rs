use clap::Parser;
use std::{thread, time::Duration};
use tracing_subscriber::EnvFilter;

use django_rs::{
    logstrategy::{LogStrategyType, default_strategies::tracing_strategy::TracingStrategy},
    taskhandler::TaskHandler,
    taskrunnable::TaskRunnable,
};

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

    let handler = TaskHandler::new(8, TracingStrategy {})?;

    for _ in 0..50 {
        handler.create_task(PrintTask::new());
    }

    thread::sleep(Duration::from_secs(10));
    println!("Active: {}", handler.get_active_workers());
    handler.shutdown()?;

    Ok(())
}
