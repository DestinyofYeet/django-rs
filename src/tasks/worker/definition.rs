use std::{
    cell::RefCell,
    sync::{
        Arc, Mutex,
        mpsc::{self, Receiver, Sender},
    },
    thread::{self, JoinHandle},
};

use tracing::trace;

use crate::tasks::{
    task::{Task, TaskState},
    worker::WorkerError,
};

pub enum WorkerCommand {
    ProcessTask(Arc<Mutex<Task>>),
    Init,
    Close,
}

pub struct Worker {
    id: u64,
    handle: RefCell<Option<JoinHandle<()>>>,
    to_thread: Sender<WorkerCommand>,
    has_task: Arc<Mutex<bool>>,
}

impl Worker {
    pub fn new(id: u64) -> Result<Self, WorkerError> {
        let (tx, rx): (Sender<WorkerCommand>, Receiver<WorkerCommand>) = mpsc::channel();

        tx.send(WorkerCommand::Init)
            .map_err(|e| WorkerError::Channel(e.to_string()))?;

        let has_task = Arc::new(Mutex::new(false));

        let has_task1 = has_task.clone();

        let handle = thread::Builder::new()
            .name(format!("Worker {id}"))
            .spawn(move || {
                let worker_log = |text: &str| {
                    trace!("[Worker {id}] {}", text);
                };

                while let Some(command) = rx.iter().next() {
                    match command {
                        WorkerCommand::ProcessTask(task) => {
                            let mut task = task.lock().expect("to get lock");

                            worker_log(&format!("Processing task {}", task.get_id()));
                            {
                                *has_task.lock().expect("to get lock") = true;
                            }
                            task.set_state(TaskState::Running);
                            task.run(id);
                            task.set_state(TaskState::Done);
                            {
                                *has_task.lock().expect("to get lock") = false;
                            }
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
            has_task: has_task1,
        })
    }

    fn send_msg(&self, command: WorkerCommand) -> Result<(), WorkerError> {
        self.to_thread
            .send(command)
            .map_err(|e| WorkerError::Channel(e.to_string()))
    }

    pub fn schedule_task(&self, task: Arc<Mutex<Task>>) -> Result<(), WorkerError> {
        self.send_msg(WorkerCommand::ProcessTask(task.clone()))
    }

    pub fn stop(&self) -> Result<(), WorkerError> {
        self.send_msg(WorkerCommand::Close)
    }

    pub fn has_task(&self) -> bool {
        *self.has_task.lock().expect("to get lock")
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
