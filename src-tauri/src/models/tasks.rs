use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]

pub struct Task {
    body: String,
    days_of_the_week: DaysOfTheWeek,
    time: Time,
    colors: String,
    repeat: bool

}
#[derive(Serialize, Deserialize, Debug, Clone)]

pub struct Time {
    pub hour: i32,
    pub minute: i32
}
#[derive(Serialize, Deserialize, Debug, Clone)]

struct DaysOfTheWeek {
    sunday: bool,
    monday: bool,
    tuesday: bool,
    wednesday: bool,
    thursday: bool,
    friday: bool,
    saturday: bool,
}


pub struct CreateTask {
    body: String,
    days_of_the_week: DaysOfTheWeek,
    time: Time,
    colors: String,
    repeat: bool
}