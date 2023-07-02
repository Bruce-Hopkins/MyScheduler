use mongodb::Collection;

use crate::models::tasks::{Task, CreateTask};

pub struct TasksService(Collection<Task>);
impl TasksService {
    pub fn new(collection: Collection<Task>) -> Self {
        Self(collection)
    }

    pub async fn create(create_task: CreateTask) {
        
    }

}
