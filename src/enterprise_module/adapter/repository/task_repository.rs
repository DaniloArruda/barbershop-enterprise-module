use chrono::Duration;
use uuid::Uuid;

use crate::{application::repository::task_repository::TaskRepository, domain::entity::task::Task};

pub struct TaskRepositoryPostgres {}

impl TaskRepository for TaskRepositoryPostgres {
    fn find_by_id(&self, task_id: Uuid) -> Option<Task> {
        Some(Task {
            id: task_id,
            description: "Moicano".to_string(),
            price: 50.0,
            duration: Duration::minutes(30),
        })
    }
}
