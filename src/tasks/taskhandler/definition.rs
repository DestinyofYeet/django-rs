use std::{
    sync::{
        Arc, Mutex,
        mpsc::{self, Sender},
    },
    thread::{self, JoinHandle},
};

use uuid::Uuid;

use crate::tasks::{
    logstrategy::LogStrategyType, task::Task, taskhandler::main_loop::MainLoopData,
};

pub(crate) enum TaskEvent {
    Shutdown,
    ProcessTask(Arc<Mutex<Task>>),
    ProcessLongTask(Arc<Mutex<Task>>),
    TaskDone(Uuid),
    RegisterSubscriber {
        for_task: Uuid,
        subscriber: Sender<TaskSubscriberEvent>,
    },

    UnregisterSubscriber {
        for_task: Uuid,
    },
}

pub enum TaskSubscriberEvent {
    CommInit,
    TaskDone,
}

pub struct TaskHandler {
    pub(super) log_strategy: LogStrategyType,
    pub(super) max_workers: u64,

    pub(super) to_handler: Sender<TaskEvent>,

    pub(super) handle: JoinHandle<()>,
}

impl TaskHandler {
    pub fn new(max_workers: u64, log_strategy: LogStrategyType) -> Self {
        let (sender, receiver) = mpsc::channel();

        let data = MainLoopData {
            recv: receiver,
            sender: sender.clone(),
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
}
