use std::future::Future;

use chrono::{DateTime, Utc};

pub fn milliseconds_between_dates(start:DateTime<Utc>,  end: DateTime<Utc>) -> i64 {
    let duration = end.signed_duration_since(start);
    duration.num_milliseconds()
}

pub async fn crontask<F, T>(end: DateTime<Utc>, callback: F) 
    where F: FnOnce() -> T,
    T: Future<Output = T> + 'static {
}

pub async fn end_of_day_scheduler(today: DateTime<Utc>) {
    
}