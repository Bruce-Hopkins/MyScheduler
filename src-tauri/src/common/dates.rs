use chrono::{DateTime, Utc, TimeZone, Datelike};

pub fn remove_hours_from_date(date: DateTime<Utc>) -> Option<DateTime<Utc>> {
    let day = date.day();
    let month = date.month();
    let year = date.year() as i32;

    chrono::Utc.with_ymd_and_hms(year, month, day, 0, 0, 0).earliest()
}
