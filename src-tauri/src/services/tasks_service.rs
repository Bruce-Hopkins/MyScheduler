 use chrono::Weekday;
use mongodb::{Collection, bson::{oid::ObjectId, doc}, results::InsertOneResult, Cursor};
use tokio_stream::StreamExt;

use crate::{models::tasks::{Task, CreateTask, WeekDay}, common::errors::{AppErrors, AppResult}};

use super::base_service::BaseService;

pub struct TasksService(BaseService<Task>);
impl TasksService {
    pub fn new(collection: Collection<Task>) -> Self {
        Self(BaseService::new(collection, "task".to_string()))
    }

    pub async fn create(&self, create_task: CreateTask) -> AppResult<InsertOneResult> {
        let task = create_task.into_model();

        self.0.create(&task).await
    }

    pub async fn get_by_id(&self, id: &ObjectId) -> AppResult<Task> {
        self.0.get_one_by_id(id).await
    }

    /** 
        Get's all the tasks based on the day passed 
    */
    pub async fn filter_by_day(&self, week_day: WeekDay) -> AppResult<Vec<Task>> {
        let doc = doc! {"days_of_the_week": doc! {
           week_day.to_str(): true 
        }};

        self.0.get_all_by(doc).await
    }

    async fn get_tasks_from_cursor(
        &self,
        mut cursor: Cursor<Task>,
    ) -> AppResult<Vec<Task>> {
        let mut response_vec: Vec<Task> = Vec::new();

        while let Some(result) = cursor.next().await {
            match result {
                Ok(doc) => {
                    response_vec.push(doc);
                }
                Err(e) => return Err(AppErrors::InternalError(e.to_string())),
            }
        }
        Ok(response_vec)
    }

    /**
     * Filters all the tasks by the day of the week
     */
    pub async fn filter_by_day_of_the_week() {
        todo!()
    }

    /**
     * Get's all the tasks and sorts by the time the tasks will happen
     */
    pub async fn get_my_tasks() {
        todo!()
    }

    /**
     * Get's the task by the id passed
     */
    pub async fn find_by_id() {
        todo!()
    }

    /**
     * Updates the entry based on the id passed
     */
    pub async fn update_by_id() {
        todo!()
    }

    /**
     * Deleted the entry based on the id passed.
     */
    pub async fn delete_by_id() {
        todo!()
    }
}

