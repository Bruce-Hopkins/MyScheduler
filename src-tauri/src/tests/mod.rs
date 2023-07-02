use mongodb::{Database, Collection};

use crate::{services::tasks_service::{TasksService, self}, models::tasks::{Task, CreateTask}, init_db, common::errors::AppResult};

mod handlers;
mod services;

struct TestServices {
    task_service: TasksService
}

struct TestBuilder {
    services: TestServices,
}

impl TestBuilder {
    pub async fn new() -> Self {
        let db = init_db().await;

        let task_col = test_collection(&db, "tasks").await;
        let task_service = TasksService::new(task_col);

        let services = TestServices {
            task_service
        };
        Self {
            services
        }
    }

    pub async fn create_task(self) -> AppResult<Self> {
        let create_task = CreateTask::default();
        self.services.task_service.create(create_task).await?;
        Ok(self)
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

