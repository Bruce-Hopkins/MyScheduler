use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]

pub struct Task {
    days_of_the_week: DaysOfTheWeek,
}
#[derive(Serialize, Deserialize, Debug, Clone)]

struct Time {
    hour: i32,
    minute: i32
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
