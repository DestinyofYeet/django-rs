use std::{
    marker::PhantomData,
    sync::{Arc, Mutex},
};

use uuid::Uuid;

use crate::tasks::{
    task::{Task, TaskState},
    taskrunnable::TaskResultable,
};

pub struct TaskRef<T> {
    task: Arc<Mutex<Task>>,
    _m: PhantomData<T>,
}

impl<T> TaskRef<T>
where
    T: TaskResultable,
{
    pub(crate) fn new(task: Arc<Mutex<Task>>) -> Self {
        Self {
            task,
            _m: PhantomData,
        }
    }

    pub fn get_result(&self) -> Option<T::Result> {
        let result = self.task.lock().expect("to get lock").get_result();
        Some(T::downcast(result?))
    }
}

impl<T> TaskRef<T> {
    pub fn get_id(&self) -> Uuid {
        self.task.lock().expect("to get lock").get_id()
    }

    pub fn get_state(&self) -> TaskState {
        self.task.lock().expect("to get lock").get_state()
    }
}
