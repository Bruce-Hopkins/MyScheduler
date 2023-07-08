 use chrono::{Weekday, Utc, TimeZone, Days};
use mongodb::{Collection, bson::{oid::ObjectId, doc}, results::{InsertOneResult, UpdateResult, DeleteResult}, Cursor};
use tokio_stream::StreamExt;

use crate::{models::tasks::{Task, CreateTask, RoutineWeekDay}, common::{errors::{AppErrors, AppResult}, dates::remove_hours_from_date}};

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


    /** 
        Get's all the tasks based on the day passed 
    */
    pub async fn filter_by_day(&self, date: chrono::DateTime<Utc>) -> AppResult<Vec<Task>> {
        let date1 = remove_hours_from_date(date).unwrap();

        // Get the dates between today and tomorrow.
        let date2 = date1.checked_add_days(Days::new(1)).unwrap();

        let doc = doc! { "start_at": { "$gte": date1, "$lte": date2 } };

        self.0.get_all_by(Some(doc)).await
    }



    /**
     * Get's all the tasks and sorts by the time the tasks will happen
     */
    pub async fn get_my_tasks(&self) -> AppResult<Vec<Task>> {
        let sort = doc! {"start_time": 1};
        self.0.get_all_by_and_sort(None, sort).await
    }

    /**
     * Get's the task by the id passed
     */
    pub async fn find_by_id(&self, id: &ObjectId) -> AppResult<Task> {
        self.0.get_one_by_id(&id).await
    }

    /**
     * Updates the entry based on the id passed
     */
    pub async fn update_by_id(&self, id:&ObjectId, create_task: CreateTask) -> AppResult<UpdateResult>{

        let doc = doc! {
            "body": create_task.body,
            "start_at": create_task.start_at,
            "end_at": create_task.end_at,
            "colors": create_task.colors,
        };
        self.0.update_by_id(id, doc).await
    }

    /**
     * Deleted the entry based on the id passed.
     */
    pub async fn delete_by_id(&self, id:&ObjectId) -> AppResult<DeleteResult> {
        self.0.delete_by_id(id).await
    }
}

