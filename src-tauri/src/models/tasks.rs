use chrono::{Date, Utc, DateTime, TimeZone, Datelike, Timelike, Days};
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]

pub struct Task {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    id: Option<ObjectId>,
    body: String,

    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    start_at: chrono::DateTime<Utc>,

    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    end_at: chrono::DateTime<Utc>,

    colors: String,

    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    created_at: chrono::DateTime<Utc>
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]

pub struct Time {
    pub hour: i32,
    pub minute: i32
}


pub enum WeekDay {
    Sunday,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
}

impl WeekDay {
    pub fn to_str(&self) -> &str {
        match self {
            WeekDay::Sunday => "sunday",
            WeekDay::Monday => "monday",
            WeekDay::Tuesday => "tuesday",
            WeekDay::Wednesday => "wednesday",
            WeekDay::Thursday => "thursday",
            WeekDay::Friday => "friday",
            WeekDay::Saturday => "saturday",
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]

pub struct CreateTask {
    body: String,
    start_at: String,
    end_at: String,
    colors: String,
}

impl CreateTask {
    pub fn into_model(self) -> Task {
        let start_at_date =  DateTime::<Utc>::from_utc(
            DateTime::parse_from_rfc3339(&self.start_at).unwrap().naive_utc(),
            Utc,
        );

        let end_at_date = DateTime::<Utc>::from_utc(
            DateTime::parse_from_rfc3339(&self.end_at).unwrap().naive_utc(),
            Utc,
        );
        Task { id: None, body: self.body, start_at: start_at_date, end_at: end_at_date, colors: self.colors, created_at: Utc::now() }
    }
}