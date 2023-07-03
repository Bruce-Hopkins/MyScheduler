use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

use super::tasks::Time;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Routine {
        #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
        id: Option<ObjectId>,
        body: String,
        days_of_the_week: DaysOfTheWeek,
        time: Time,
        colors: String,
        repeat: bool
    
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateRoutine {
        #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
        id: Option<ObjectId>,
        body: String,
        days_of_the_week: DaysOfTheWeek,
        time: Time,
        colors: String,
    
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