use chrono::{Utc, Duration, Days, TimeZone, Timelike};

use crate::{handlers::cron::{milliseconds_between_dates, CronjobHandler}, models::tasks::Task};

#[tokio::test]
async fn milliseconds_between_should_be_accurate() {
    let today = Utc.with_ymd_and_hms(2014, 7, 8, 9, 10, 11).unwrap();
    let tomorrow = today.checked_add_days(Days::new(1)).unwrap();
    let result = milliseconds_between_dates(today, tomorrow);

    // A full day in miliseconds
    assert_eq!(86400000, result);
}

#[tokio::test]
async fn cron_handler_hours() {
    let task = Task::default();

    let time = task.end_at();
    let mut cronhandler = CronjobHandler::new();

    cronhandler.add_task(task);
    let has_cronjob = cronhandler.contains_hours(time.minute(), time.hour()).await;

    assert!(has_cronjob)
}

