use uuid::Uuid;

use crate::tasks::task::Runnable;

pub struct QueueItem {
    id: Uuid,
    runnable: Runnable,
}
