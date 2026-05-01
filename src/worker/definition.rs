use std::{
    cell::RefCell,
    sync::mpsc::{self, Receiver, Sender},
    thread::{self, JoinHandle},
};

use tracing::debug;

use crate::{logstrategy::LogStrategy, task::Task, worker::WorkerError};

pub enum WorkerCommand<T>
where
    T: LogStrategy,
{
    ProcessTask(Task<T>),
    Init,
    Close,
}

pub struct Worker<T>
where
    T: LogStrategy,
{
    id: u64,
    handle: RefCell<Option<JoinHandle<()>>>,
    to_thread: Sender<WorkerCommand<T>>,
}

impl<T> Worker<T>
where
    T: LogStrategy + Send + Sync + 'static,
{
    pub fn new(id: u64) -> Result<Self, WorkerError> {
        let (tx, rx): (Sender<WorkerCommand<T>>, Receiver<WorkerCommand<T>>) = mpsc::channel();

        tx.send(WorkerCommand::Init)
            .map_err(|e| WorkerError::Channel(e.to_string()))?;

        let handle = thread::Builder::new()
            .name(format!("Worker {id}"))
            .spawn(move || {
                let worker_log = |text: &str| {
                    debug!("[Worker {id}] {}", text);
                };

                while let Some(command) = rx.iter().next() {
                    match command {
                        WorkerCommand::ProcessTask(mut task) => {
                            worker_log(&format!("Processing task {}", task.get_id()));
                            task.run(id);
                        }

                        WorkerCommand::Init => worker_log("init"),
                        WorkerCommand::Close => break,
                    }
                }

                worker_log("exit")
            })
            .unwrap();

        Ok(Self {
            id,
            handle: RefCell::new(Some(handle)),
            to_thread: tx,
        })
    }

    fn send_msg(&self, command: WorkerCommand<T>) -> Result<(), WorkerError> {
        self.to_thread
            .send(command)
            .map_err(|e| WorkerError::Channel(e.to_string()))
    }

    pub fn schedule_task(&self, task: Task<T>) -> Result<(), WorkerError> {
        self.send_msg(WorkerCommand::ProcessTask(task))
    }

    pub fn stop(&self) -> Result<(), WorkerError> {
        self.send_msg(WorkerCommand::Close)
    }

    pub fn wait_for_join_handle(&self) -> Result<(), WorkerError> {
        let handle = self.handle.borrow_mut().take().unwrap();
        handle.join().map_err(|_| WorkerError::Join)
    }

    pub fn is_running(&self) -> bool {
        if let Some(handle) = self.handle.borrow().as_ref() {
            return !handle.is_finished();
        }

        false
    }

    pub fn get_id(&self) -> u64 {
        self.id
    }
}
