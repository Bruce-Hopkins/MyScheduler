use crate::tests::TestBuilder;

#[tokio::test]
async fn create_task_should_succeed() {
    let test_builder = TestBuilder::new().await;
    test_builder.create_task().await.expect("Failed to create a task");
}
