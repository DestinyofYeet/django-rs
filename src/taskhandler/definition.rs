use std::{
    collections::VecDeque,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use thiserror::Error;
use tracing::{error, warn};

use crate::{
    logstrategy::LogStrategy,
    task::{Runnable, Task},
    worker::{Worker, WorkerError},
};

#[derive(Error, Debug)]
pub enum TaskError {
    #[error("{0}")]
    Worker(#[from] WorkerError),
}

pub struct TaskHandler<T>
where
    T: LogStrategy + Send,
{
    queue: Arc<Mutex<VecDeque<Task<T>>>>,
    workers: Arc<Mutex<Vec<Worker<T>>>>,
    logger: Arc<T>,
}

impl<T> TaskHandler<T>
where
    T: LogStrategy + Send + Sync + 'static,
{
    pub fn new(workers: u64, logger: T) -> Result<Self, TaskError> {
        let mut workers_vec = Vec::new();
        for id in 0..workers {
            workers_vec.push(Worker::new(id)?)
        }

        let value = Self {
            logger: Arc::new(logger),
            queue: Arc::new(Mutex::new(VecDeque::new())),
            workers: Arc::new(Mutex::new(workers_vec)),
        };

        let q1 = value.queue.clone();
        let w1 = value.workers.clone();

        thread::spawn(move || TaskHandler::manage_workers(q1, w1, workers));

        Ok(value)
    }

    pub fn queue_task(&mut self, task: Task<T>) {
        self.queue.lock().expect("to get lock").push_back(task);
    }

    pub fn manage_workers(
        queue: Arc<Mutex<VecDeque<Task<T>>>>,
        workers: Arc<Mutex<Vec<Worker<T>>>>,
        max_workers: u64,
    ) -> ! {
        let mut current_worker_id = max_workers;
        loop {
            let mut queue = queue.lock().expect("to get queue lock");

            let existing_workers = TaskHandler::static_get_active_workers(workers.clone()) as u64;

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

            thread::sleep(Duration::from_millis(500));
        }
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
        TaskHandler::<T>::static_get_active_workers(self.workers.clone())
    }

    fn static_get_active_workers(workers: Arc<Mutex<Vec<Worker<T>>>>) -> usize {
        let workers = workers.lock().expect("to get worker lock");
        workers
            .iter()
            .map(|e| if e.is_running() { 1 } else { 0 })
            .sum()
    }

    pub fn create_task(&self, taskable: Runnable) {
        let task = Task::new(taskable, self.logger.clone());
        self.queue
            .lock()
            .expect("to get queue lock")
            .push_back(task);
    }
}
