use std::{
    sync::mpsc::{self, Receiver, Sender},
    thread::{self, JoinHandle},
};

use tracing::{debug, info};

use crate::tasks::{logstrategy::LogStrategyType, taskhandler::TaskHandlerError, worker::Worker};

struct MainLoopData {
    recv: Receiver<TaskEvent>,
    max_workers: u64,
}

pub(crate) enum TaskEvent {
    Shutdown,
}

pub struct TaskHandler {
    log_strategy: LogStrategyType,
    max_workers: u64,

    to_handler: Sender<TaskEvent>,

    handle: JoinHandle<()>,
}

impl TaskHandler {
    pub fn new(max_workers: u64, log_strategy: LogStrategyType) -> Self {
        let (sender, receiver) = mpsc::channel();

        let data = MainLoopData {
            recv: receiver,
            max_workers,
        };

        let handle = thread::Builder::new()
            .name("TaskHandler".to_string())
            .spawn(move || {
                TaskHandler::main_loop(data);
            })
            .unwrap();

        Self {
            max_workers,
            log_strategy,
            to_handler: sender,
            handle,
        }
    }

    pub fn shutdown(self) -> Result<(), TaskHandlerError> {
        self.to_handler.send(TaskEvent::Shutdown)?;
        Ok(())
    }

    fn main_loop(data: MainLoopData) {
        let mut workers: Vec<Worker> = Vec::with_capacity(data.max_workers as usize);

        for i in 0..data.max_workers {
            workers.push(Worker::new(i).expect("to create workers"));
        }

        while let Some(command) = data.recv.iter().next() {
            match command {
                TaskEvent::Shutdown => break,
            }
        }

        debug!("TaskHandler exited")
    }
}
