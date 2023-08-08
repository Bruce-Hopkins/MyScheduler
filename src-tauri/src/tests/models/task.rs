use chrono::TimeZone;

use crate::models::tasks::{TaskList, Task};




#[tokio::test]
async fn get_todays_task_should_succeed() {
    let mut tasks = vec![];

    let mut task_one = Task::default();
    let start_date = chrono::Utc
        .with_ymd_and_hms(2020, 6, 11, 0, 0, 0)
        .earliest().unwrap();
    let end_date = chrono::Utc
    .with_ymd_and_hms(2020, 6, 11, 3, 0, 0)
    .earliest().unwrap();

    task_one.set_start_at(start_date);
    task_one.set_end_at(end_date);

    let mut task_two = Task::default();
    let start_date = chrono::Utc
        .with_ymd_and_hms(2020, 6, 11, 0, 0, 0)
        .earliest().unwrap();
    let end_date = chrono::Utc
    .with_ymd_and_hms(2020, 6, 11, 2, 0, 0)
    .earliest().unwrap();
    task_two.set_start_at(start_date);
    task_two.set_end_at(end_date);

    let mut task_three = Task::default();
    let start_date = chrono::Utc
        .with_ymd_and_hms(2020, 6, 11, 4, 0, 0)
        .earliest().unwrap();
    let end_date = chrono::Utc
    .with_ymd_and_hms(2020, 6, 11, 5, 0, 0)
    .earliest().unwrap();
    task_three.set_start_at(start_date);
    task_three.set_end_at(end_date);

    tasks.push(task_one);
    tasks.push(task_two);
    tasks.push(task_three);



    let list = TaskList::new(tasks);
    let group = list.group_tasks();

    assert_eq!(group.len(), 2);
}
