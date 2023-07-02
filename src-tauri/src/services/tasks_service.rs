use mongodb::{Collection, bson::{oid::ObjectId, doc}};

use crate::{models::tasks::{Task, CreateTask}, common::errors::{AppErrors, AppResult}};

pub struct TasksService(Collection<Task>);
impl TasksService {
    pub fn new(collection: Collection<Task>) -> Self {
        Self(collection)
    }

    pub async fn create(&self, create_task: CreateTask) -> AppResult<()> {
        let task = create_task.into_model();
        let result = self.0.insert_one(task, None).await;
        match result {
            Ok(_r) => Ok(()),
            Err(e) => Err(AppErrors::InternalError(format!("Failed to create a new task: {}", e.to_string())))
        }
    }

    pub async fn get_by_id(&self, id: ObjectId) -> AppResult<Task> {
        let task = self.0.find_one(doc! {"_id": id}, None).await;
        let task = AppErrors::from_unknown_result(task, "Could not run find query on the task")?;
        match task {
            Some(v) => Ok(v),
            None => Err(AppErrors::NotFound)
        }
    }

}
