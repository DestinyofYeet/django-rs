use crate::tasks::worker_logger::WorkerLogger;

impl WorkerLogger {
    pub fn warn(&self, message: &str) {
        self.logger.warn(self.worker_id, message);
    }

    pub fn error(&self, message: &str) {
        self.logger.error(self.worker_id, message);
    }

    pub fn info(&self, message: &str) {
        self.logger.info(self.worker_id, message);
    }
}
