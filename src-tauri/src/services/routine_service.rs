use bson::{oid::ObjectId, doc};
use chrono::Utc;
use mongodb::{Collection, results::{InsertOneResult, DeleteResult, UpdateResult}};

use crate::{models::routine::{Routine, CreateRoutine}, common::errors::AppResult};

use super::base_service::BaseService;


// TODO: Refactor common methods into a trait
pub struct RoutineService(BaseService<Routine>);
impl RoutineService {
    pub fn new(collection: Collection<Routine>) -> Self {
        Self(BaseService::new(collection, "routine".to_string()))
    }

    pub async fn create(&self, create_routine: CreateRoutine) -> AppResult<InsertOneResult> {
        let task = create_routine.into_model();

        self.0.create(&task).await
    }


    /** 
        Get's all the tasks based on the day passed 
    */
    pub async fn filter_by_day(&self, date: chrono::DateTime<Utc>) -> AppResult<Vec<Routine>> {
        todo!()
    }



    /**
     * Get's all the routines and sorts by the time the tasks will happen
     */
    pub async fn get_my_tasks(&self) -> AppResult<Vec<Routine>> {
        let sort = doc! {"created_at": 1};
        self.0.get_all_by_and_sort(None, sort).await
    }

    /**
     * Get's the Routine by the id passed
     */
    pub async fn find_by_id(&self, id: &ObjectId) -> AppResult<Routine> {
        self.0.get_one_by_id(&id).await
    }

    /**
     * Updates the entry based on the id passed
     */
    pub async fn update_by_id(&self, id:&ObjectId, create_routine: CreateRoutine) -> AppResult<UpdateResult>{

        let doc = doc! {
            "body": create_routine.body,
            "start_at": create_routine.start_at.to_bson(),
            "end_at": create_routine.end_at.to_bson(),
            "colors": create_routine.colors,
        };
        self.0.update_by_id(id, doc).await
    }

    /**
     * Deleted the entry based on the id passed.
     */
    pub async fn delete_by_id(&self, id:&ObjectId) -> AppResult<DeleteResult> {
        self.0.delete_by_id(id).await
    }

    pub async fn get_by_id(&self, id: &ObjectId) -> AppResult<Routine> {
        self.0.get_one_by_id(id).await
    }
}