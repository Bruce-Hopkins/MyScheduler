use chrono::Weekday;
use mongodb::{Collection, bson::{oid::ObjectId, doc}, results::InsertOneResult, Cursor};
use tokio_stream::StreamExt;

use crate::{models::tasks::{Task, CreateTask, WeekDay}, common::errors::{AppErrors, AppResult}};

pub struct TasksService(Collection<Task>);
impl TasksService {
    pub fn new(collection: Collection<Task>) -> Self {
        Self(collection)
    }

    pub async fn create(&self, create_task: CreateTask) -> AppResult<InsertOneResult> {
        let task = create_task.into_model();
        let result = self.0.insert_one(task, None).await;
        match result {
            Ok(r) => Ok(r),
            Err(e) => Err(AppErrors::InternalError(format!("Failed to create a new task: {}", e.to_string())))
        }
    }

    pub async fn get_by_id(&self, id: &ObjectId) -> AppResult<Task> {
        let task = self.0.find_one(doc! {"_id": id}, None).await;
        let task = AppErrors::from_unknown_result(task, "Could not run find query on the task")?;
        match task {
            Some(v) => Ok(v),
            None => Err(AppErrors::NotFound)
        }
    }

    /** 
        Get's all the tasks based on the day passed 
    */
    pub async fn filter_by_day(&self, week_day: WeekDay) -> AppResult<Vec<Task>> {
        let doc = doc! {"days_of_the_week": doc! {
           week_day.to_str(): true 
        }};
        let cursor = self.0.find(doc, None).await;
        let cursor = AppErrors::from_unknown_result(cursor, "Failed to get task cursor")?;
        self.get_tasks_from_cursor(cursor).await
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
