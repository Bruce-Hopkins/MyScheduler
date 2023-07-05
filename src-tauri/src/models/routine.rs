use chrono::Utc;
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

use super::tasks::Time;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Routine {
        #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
        id: Option<ObjectId>,
        body: String,
        days_of_the_week: DaysOfTheWeek,
        start_at: Time,
        end_at: Time,
        colors: String,
        
        #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
        created_at: chrono::DateTime<Utc>
    
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateRoutine {
        pub body: String,
        pub days_of_the_week: DaysOfTheWeek,
        pub time: Time,
        pub colors: String,
        pub start_at: Time,
        pub end_at: Time,
}

impl CreateRoutine {
        pub fn into_model(self) -> Routine {
                Routine { id: None, body: self.body, days_of_the_week: self.days_of_the_week, start_at: self.start_at, end_at: self.end_at, colors: self.colors, created_at: Utc::now() }
        }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]

pub struct DaysOfTheWeek {
    sunday: bool,
    monday: bool,
    tuesday: bool,
    wednesday: bool,
    thursday: bool,
    friday: bool,
    saturday: bool,
}