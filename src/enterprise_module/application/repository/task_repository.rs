#[cfg(test)]
use mockall::automock;
use uuid::Uuid;

use crate::domain::entity::task::Task;

#[cfg_attr(test, automock)]
pub trait TaskRepository {
    fn find_by_id(&self, task_id: Uuid) -> Option<Task>;
}
