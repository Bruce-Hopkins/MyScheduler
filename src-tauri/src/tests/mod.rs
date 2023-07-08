use chrono::Utc;
use mongodb::{Database, Collection, bson::oid::ObjectId};

use crate::{services::{tasks_service::{TasksService, self}, routine_service::RoutineService}, models::{tasks::{Task, CreateTask}, routine::CreateRoutine}, init_db, common::errors::AppResult};

mod handlers;
mod services;

struct TestServices {
    task_service: TasksService,
    routine_service: RoutineService
}

struct TestBuilder {
    services: TestServices,
}

impl TestBuilder {
    pub async fn new() -> Self {
        let db = init_db().await;

        let task_col = test_collection(&db, "tasks").await;
        let task_service = TasksService::new(task_col);

        let task_col = test_collection(&db, "routine").await;
        let routine_service: RoutineService = RoutineService::new(task_col);

        let services = TestServices {
            task_service,
            routine_service,
        };
        Self {
            services
        }
    }

    pub async fn create_task(&self) -> AppResult<ObjectId> {
        let create_task = CreateTask::default();
        let id = self.services.task_service.create(create_task).await?;
        Ok(id.inserted_id.as_object_id().unwrap())
    }

    pub async fn create_routine(&self) -> AppResult<ObjectId> {
        let create_routine = CreateRoutine::default_from_date(Utc::now());
        println!("The routine is: {:?}", create_routine);

        let id = self.services.routine_service.create(create_routine).await?;
        Ok(id.inserted_id.as_object_id().unwrap())
    }
}

/**
 * Creates a collection for testing.
 * 
 * Side effect: drops the previous collection of that name
 */
pub async fn test_collection<T>(db: &Database, col_name: &str) -> Collection<T> {
    let collection: Collection<T> = db.collection(col_name);
    collection
        .drop(None)
        .await
        .expect("could not drop collection");
    db.collection(col_name)
}

