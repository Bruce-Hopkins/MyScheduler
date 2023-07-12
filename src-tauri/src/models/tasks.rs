use bson::{Document, doc};
use chrono::{Date, Utc, DateTime, TimeZone, Datelike, Timelike, Days};
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

use crate::common::dates::remove_hours_from_date;

#[derive(Serialize, Deserialize, Debug, Clone)]

pub struct Task {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    id: Option<ObjectId>,
    body: String,
    status: String,

    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    start_at: chrono::DateTime<Utc>,

    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    end_at: chrono::DateTime<Utc>,

    colors: String,

    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    created_at: chrono::DateTime<Utc>
}

impl Task {
    fn into_res(self) -> TaskRes {
        TaskRes { 
            id: self.id.unwrap().to_hex(), 
            body: self.body, 
            start_time: Time::from_date(self.start_at), 
            end_time: Time::from_date(self.end_at), 
            colors: self.colors, 
            type_task: String::from("task") 
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]

pub struct Time {
    pub hour: i32,
    pub minute: i32
}

impl Time {
    pub fn to_bson(self) -> Document {
       doc! {
        "hour": self.hour,
        "minute": self.minute
       }
    } 

    pub fn from_date(date: chrono::DateTime<Utc>) -> Time {
        Self {
            hour: date.hour() as i32,
            minute: date.minute() as i32 
        }
    }
}

pub enum RoutineWeekDay {
    Sunday,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
}

impl RoutineWeekDay {
    pub fn to_str(&self) -> &str {
        match self {
            RoutineWeekDay::Sunday => "sunday",
            RoutineWeekDay::Monday => "monday",
            RoutineWeekDay::Tuesday => "tuesday",
            RoutineWeekDay::Wednesday => "wednesday",
            RoutineWeekDay::Thursday => "thursday",
            RoutineWeekDay::Friday => "friday",
            RoutineWeekDay::Saturday => "saturday",
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]

pub struct CreateTask {
    pub body: String,
    pub start_at: String,
    pub end_at: String,
    pub colors: String,
}

impl Default for CreateTask {
    fn default() -> Self {
        Self { body: String::new(), start_at: Utc::now().to_rfc3339(), end_at: Utc::now().to_rfc3339(), colors: String::from("#A3D9FF") }
    }
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

        Task { 
            id: None, 
            body: self.body, 
            start_at: start_at_date, 
            end_at: end_at_date, 
            colors: self.colors, 
            created_at: Utc::now(),
            status: String::from("active")
        }
    }
}

pub struct TaskRes {
    id: String,
    body: String,
    start_time: Time,
    end_time: Time,
    colors: String,
    type_task: String
}