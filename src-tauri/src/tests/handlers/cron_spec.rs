use chrono::{Utc, Duration, Days, TimeZone};

use crate::handlers::cron::milliseconds_between_dates;

#[tokio::test]
async fn milliseconds_between_should_be_accurate() {
    let today = Utc.with_ymd_and_hms(2014, 7, 8, 9, 10, 11).unwrap();
    let tomorrow = today.checked_add_days(Days::new(1)).unwrap();
    let result = milliseconds_between_dates(today, tomorrow);

    // A full day in miliseconds
    assert_eq!(86400000, result);

}