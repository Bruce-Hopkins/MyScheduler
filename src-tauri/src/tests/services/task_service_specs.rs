use mongodb::bson::oid::ObjectId;

use crate::tests::TestBuilder;


async fn initialize() -> ObjectId {
    let test_builder = TestBuilder::new().await;
    test_builder.create_task().await.expect("Failed to create a task")
}


#[tokio::test]
async fn get_task_should_succeed() {
    let test_bulder = TestBuilder::new().await;
    let id = initialize().await;

    let result = test_bulder.services.task_service.get_by_id(&id).await;

    assert!(result.is_ok())

}
