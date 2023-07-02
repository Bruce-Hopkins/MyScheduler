use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]

pub struct Task {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    id: Option<ObjectId>,
    body: String,
    days_of_the_week: DaysOfTheWeek,
    time: Time,
    colors: String,
    repeat: bool

}
#[derive(Serialize, Deserialize, Debug, Clone, Default)]

pub struct Time {
    pub hour: i32,
    pub minute: i32
}
#[derive(Serialize, Deserialize, Debug, Clone, Default)]

struct DaysOfTheWeek {
    sunday: bool,
    monday: bool,
    tuesday: bool,
    wednesday: bool,
    thursday: bool,
    friday: bool,
    saturday: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]

pub struct CreateTask {
    body: String,
    days_of_the_week: DaysOfTheWeek,
    time: Time,
    colors: String,
    repeat: bool
}

impl CreateTask {
    pub fn into_model(self) -> Task {
        Task { id: None, body: self.body, days_of_the_week: self.days_of_the_week, time: self.time, colors: self.colors, repeat: self.repeat }
    }
}