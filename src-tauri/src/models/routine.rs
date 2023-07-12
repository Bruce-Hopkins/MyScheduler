use chrono::{Utc, Datelike};
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use chrono::{Weekday};

use super::tasks::{Time, RoutineWeekDay};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Routine {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    id: Option<ObjectId>,
    body: String,
    days_of_the_week: DaysOfTheWeek,
    start_at: Time,
    end_at: Time,
    colors: String,
    status: String,
    
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    created_at: chrono::DateTime<Utc>
    
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
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
        Routine { 
            id: None, 
            body: self.body, 
            days_of_the_week: self.days_of_the_week, 
            start_at: self.start_at, 
            end_at: self.end_at, 
            colors: self.colors, 
            created_at: Utc::now() ,
            status: String::from("active")
        }
    }

    pub fn default_from_date(date: chrono::DateTime<Utc>) -> CreateRoutine {
        let mut routine = Self::default();
        let days_of_week = DaysOfTheWeek::from_weeK_day(date.weekday());

        routine.days_of_the_week = days_of_week;
        routine
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

impl DaysOfTheWeek {
    pub fn from_weeK_day(day: Weekday) -> DaysOfTheWeek {
        let mut days_of_the_week = DaysOfTheWeek::default();
        match day {
            Weekday::Mon => days_of_the_week.monday = true,
            Weekday::Tue => days_of_the_week.tuesday = true,
            Weekday::Wed => days_of_the_week.wednesday = true,
            Weekday::Thu => days_of_the_week.thursday = true,
            Weekday::Fri => days_of_the_week.friday = true,
            Weekday::Sat => days_of_the_week.saturday = true,
            Weekday::Sun => days_of_the_week.sunday = true,
        }

        days_of_the_week

    }
}

pub fn week_day_from_str(week_day: Weekday) -> &'static str {
    match week_day {
        Weekday::Sun => "sunday",
        Weekday::Mon => "monday",
        Weekday::Tue => "tuesday",
        Weekday::Wed => "wednesday",
        Weekday::Thu => "thursday",
        Weekday::Fri => "friday",
        Weekday::Sat => "saturday",
    }
}