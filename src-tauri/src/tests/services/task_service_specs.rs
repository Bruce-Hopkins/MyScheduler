use chrono::{Days, Utc};
use mongodb::bson::oid::ObjectId;

use crate::tests::TestBuilder;

async fn initialize() -> ObjectId {
    let test_builder = TestBuilder::new().await;
    test_builder
        .create_task()
        .await
        .expect("Failed to create a task")
}

#[tokio::test]
async fn get_task_should_succeed() {
    let test_bulder = TestBuilder::new().await;
    let id = initialize().await;

    let result = test_bulder.services.task_service.find_by_id(&id).await;

    assert!(result.is_ok())
}

#[tokio::test]
async fn filter_task_by_day_should_succeed() {
    let test_bulder = TestBuilder::new().await;
    initialize().await;

    let result = test_bulder
        .services
        .task_service
        .filter_by_day(Utc::now())
        .await;
    assert!(result.is_ok());

    let yesterday = Utc::now().checked_sub_days(Days::new(1)).unwrap();
    let result = test_bulder
        .services
        .task_service
        .filter_by_day(yesterday)
        .await;

    // There are no tasks for yesterday
    assert!(result.is_err());

    let tomorrow = Utc::now().checked_add_days(Days::new(1)).unwrap();
    let result = test_bulder
        .services
        .task_service
        .filter_by_day(tomorrow)
        .await;

    // There are no tasks for tomorrow
    assert!(result.is_err());
}
