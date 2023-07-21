use bson::oid::ObjectId;
use chrono::Utc;

use crate::{models::routine, tests::TestBuilder};

async fn initialize() -> ObjectId {
    let test_builder = TestBuilder::new().await;
    test_builder
        .create_routine()
        .await
        .expect("Failed to create a task")
}
#[tokio::test]
async fn get_task_should_succeed() {
    let test_bulder = TestBuilder::new().await;
    let id = initialize().await;

    let result = test_bulder
        .services
        .routine_service
        .get_by_id(&id)
        .await
        .unwrap();

    // assert!(result.is_ok());
}

#[tokio::test]
async fn get_todays_task_should_succeed() {
    let test_bulder = TestBuilder::new().await;
    let id = initialize().await;

    let result = test_bulder
        .services
        .routine_service
        .get_by_id(&id)
        .await
        .unwrap();

    let result = test_bulder
        .services
        .routine_service
        .filter_by_day(Utc::now())
        .await
        .unwrap();

    // assert!(result.is_ok());
}
