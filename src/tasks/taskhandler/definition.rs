use std::{
    collections::VecDeque,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use thiserror::Error;
use tracing::{error, trace, warn};
use uuid::Uuid;

use crate::{
    tasks::logstrategy::{LogStrategy, LogStrategyType},
    tasks::task::{Runnable, Task},
    tasks::worker::{Worker, WorkerError},
};

#[derive(Error, Debug)]
pub enum TaskError {
    #[error("{0}")]
    Worker(#[from] WorkerError),
}

pub struct TaskHandler {
    queue: Arc<Mutex<VecDeque<Arc<Mutex<Task>>>>>,
    workers: Arc<Mutex<Vec<Worker>>>,
    logger: LogStrategyType,
    shutdown: Arc<Mutex<bool>>,
}

impl TaskHandler {
    pub fn new(
        workers: u64,
        logger: impl LogStrategy + Send + Sync + 'static,
    ) -> Result<Self, TaskError> {
        let mut workers_vec = Vec::new();
        for id in 0..workers {
            workers_vec.push(Worker::new(id)?)
        }

        let shutdown = Arc::new(Mutex::new(false));

        let value = Self {
            logger: Arc::new(logger),
            queue: Arc::new(Mutex::new(VecDeque::new())),
            workers: Arc::new(Mutex::new(workers_vec)),
            shutdown: shutdown.clone(),
        };

        let q1 = value.queue.clone();
        let w1 = value.workers.clone();

        thread::spawn(move || TaskHandler::manage_workers(q1, w1, workers, shutdown));

        Ok(value)
    }

    // pub(crate) fn queue_task(&mut self, task: Task) {
    //     self.queue
    //         .lock()
    //         .expect("to get lock")
    //         .push_back(Arc::new(Mutex::new(task)));
    // }

    pub fn manage_workers(
        queue: Arc<Mutex<VecDeque<Arc<Mutex<Task>>>>>,
        workers: Arc<Mutex<Vec<Worker>>>,
        max_workers: u64,
        shutdown: Arc<Mutex<bool>>,
    ) {
        let mut current_worker_id = max_workers;
        loop {
            let shutdown = shutdown.lock().expect("to get shutdown lock");

            if *shutdown {
                break;
            }

            drop(shutdown);

            trace!("Getting queue lock");
            let mut queue = queue.lock().expect("to get queue lock");

            let existing_workers = TaskHandler::static_get_active_workers(workers.clone()) as u64;

            trace!("Getting worker lock");
            let mut workers_lock = workers.lock().expect("to get worker lock");

            if existing_workers != max_workers {
                let diff = max_workers - existing_workers;

                for worker in workers_lock.iter() {
                    if !worker.is_running() {
                        warn!(
                            "worker {} is not running! It probably crashed",
                            worker.get_id()
                        );
                    }
                }

                workers_lock.retain(|e| e.is_running());

                for _ in 0..diff {
                    match Worker::new(current_worker_id) {
                        Ok(value) => {
                            workers_lock.push(value);

                            warn!("Created new worker with new id {current_worker_id}");

                            current_worker_id += 1;
                        }
                        Err(e) => error!("Failed to Create new worker: {e}"),
                    };
                }
            }

            while let Some(task) = queue.pop_front() {
                let random_worker = rand::random_range(0..workers_lock.len());
                let worker = &workers_lock[random_worker as usize];
                match worker.schedule_task(task) {
                    Ok(_) => {}
                    Err(e) => {
                        eprintln!("Failed to schedule task: {e}");
                    }
                };
            }

            drop(queue);
            drop(workers_lock);
            trace!("Release worker and queue lock");

            thread::sleep(Duration::from_millis(500));
        }

        trace!("Worker manager exited!");
    }

    pub fn shutdown(self) -> Result<(), TaskError> {
        loop {
            let queue = self.queue.lock().expect("to get lock");

            if queue.is_empty() {
                break;
            }

            drop(queue);
            thread::sleep(Duration::from_millis(500));
        }

        *self.shutdown.lock().expect("to get shutdown lock") = true;

        let workers = self.workers.lock().expect("to get lock");

        for worker in workers.iter() {
            worker.stop()?
        }

        for worker in workers.iter() {
            worker.wait_for_join_handle()?
        }

        Ok(())
    }

    pub fn get_active_workers(&self) -> usize {
        TaskHandler::static_get_active_workers(self.workers.clone())
    }

    fn static_get_active_workers(workers: Arc<Mutex<Vec<Worker>>>) -> usize {
        let workers = workers.lock().expect("to get worker lock");
        workers
            .iter()
            .map(|e| if e.is_running() { 1 } else { 0 })
            .sum()
    }

    pub fn spawn_task(&self, taskable: Runnable) -> Arc<Mutex<Task>> {
        let task = Arc::new(Mutex::new(Task::new(taskable, self.logger.clone())));
        trace!("Getting queue lock");
        self.queue
            .lock()
            .expect("to get queue lock")
            .push_back(task.clone());
        trace!("Release queue lock");
        task
    }

    pub fn is_done(&self, task_id: Uuid) -> bool {
        if self
            .queue
            .lock()
            .expect("to get queue lock")
            .iter()
            .find(|e| e.lock().expect("to get task lock").get_id() == task_id)
            .is_some()
        {
            return false;
        }

        for worker in self.workers.lock().expect("to get worker lock").iter() {
            if worker.get_task() == Some(task_id) {
                return false;
            }
        }

        true
    }
}
