use std::cmp::Ordering;

use bson::{doc, Document};
use chrono::{Date, DateTime, Datelike, Days, TimeZone, Timelike, Utc};
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

use crate::common::dates::remove_hours_from_date;

#[derive(Debug, Clone)]
pub struct TaskGroup(Vec<Task>);
impl TaskGroup {
    fn new() -> Self {
        Self(Vec::new())
    }

    fn add(&mut self, task: Task) {
        self.0.push(task)
    }

    fn clear(&mut self) {
        self.0 = vec![];
    }

    pub fn into_res(self) -> Vec<TaskRes> {
        self.0.into_iter().map(|task| task.into_res()).collect()
    }
}

pub struct TaskGroupList(Vec<TaskGroup>);

impl TaskGroupList {
    fn new() -> Self {
        Self(Vec::new())
    }

    fn add(&mut self, task: TaskGroup) {
        self.0.push(task)
    }

    pub fn into_res(self) -> Vec<Vec<TaskRes>> {
        self.0.into_iter().map(|task| task.into_res()).collect()
    }
}

pub struct TaskList(Vec<Task>);

impl TaskList {
    pub fn new(tasks: Vec<Task>) -> Self {
        TaskList(tasks)
    }
    pub fn into_model(self) -> Vec<Task> {
        self.0
    }
    pub fn into_res(self) -> Vec<TaskRes> {
        self.0.into_iter().map(|task| task.into_res()).collect()
    }

    pub fn group_tasks(self) -> TaskGroupList {
        let mut tasks = self.0;
        tasks.sort();

        let mut group = TaskGroup::new();
        let mut group_lists = TaskGroupList::new();

        for i in 0..tasks.len() {
            let task = tasks.get(i).unwrap();
            group.add(task.clone());

            let next_task = tasks.get(i + 1);

            if let Some(value) = next_task {
                if value.start_at > task.end_at {
                    group_lists.add(group.clone());
                    group.clear();
                }
            } else {
                group_lists.add(group.clone());
                group.clear();
            }
        }

        group_lists
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TaskRes {
    id: String,
    body: String,
    status: String,
    start_at: String,
    end_at: String,
    color: String,
    created_at: String,
}

#[derive(Serialize, Eq, Deserialize, Debug, Clone)]

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
    created_at: chrono::DateTime<Utc>,
}

impl Task {
    fn into_res(self) -> TaskRes {
        TaskRes {
            id: self.id.unwrap().to_hex(),
            body: self.body,
            start_at: self.start_at.to_rfc3339(),
            end_at: self.end_at.to_rfc3339(),
            color: self.colors,
            status: self.status,
            created_at: self.created_at.to_rfc3339(),
        }
    }

    pub fn end_at(&self) -> DateTime<Utc> {
        self.end_at
    }

    pub fn set_start_at(&mut self, date: DateTime<Utc>) {
        self.start_at = date;
    }

    pub fn set_end_at(&mut self, date: DateTime<Utc>) {
        self.end_at = date;
    }

    pub fn id(&self) -> ObjectId {
        let id = &self.id.unwrap();
        id.clone()
    }
}

impl Ord for Task {
    fn cmp(&self, other: &Self) -> Ordering {
        self.start_at.cmp(&other.start_at)
    }
}

impl PartialOrd for Task {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Task {
    fn eq(&self, other: &Self) -> bool {
        self.start_at == other.start_at
    }
}

impl Default for Task {
    fn default() -> Task {
        Task {
            id: Some(ObjectId::default()),
            body: String::default(),
            status: String::default(),
            start_at: Utc::now(),
            end_at: Utc::now(),
            colors: String::default(),
            created_at: Utc::now(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]

pub struct Time {
    pub hour: i32,
    pub minute: i32,
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
            minute: date.minute() as i32,
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
        Self {
            body: String::new(),
            start_at: Utc::now().to_rfc3339(),
            end_at: Utc::now().to_rfc3339(),
            colors: String::from("#A3D9FF"),
        }
    }
}

impl CreateTask {
    pub fn into_model(self) -> Task {
        let start_at_date = DateTime::<Utc>::from_utc(
            DateTime::parse_from_rfc3339(&self.start_at)
                .unwrap()
                .naive_utc(),
            Utc,
        );

        let end_at_date = DateTime::<Utc>::from_utc(
            DateTime::parse_from_rfc3339(&self.end_at)
                .unwrap()
                .naive_utc(),
            Utc,
        );

        Task {
            id: None,
            body: self.body,
            start_at: start_at_date,
            end_at: end_at_date,
            colors: self.colors,
            created_at: Utc::now(),
            status: String::from("active"),
        }
    }
}
