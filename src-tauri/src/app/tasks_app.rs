use std::sync::Arc;

use bson::{oid::ObjectId, DateTime};
use chrono::{NaiveDate, NaiveDateTime, Utc};
use tokio::sync::Mutex;

use crate::{
    common::errors::DBResult,
    models::tasks::{CreateTask, Task, TaskRes, Time},
    AppState,
};

type AppStateRef = Arc<AppState>;

type AppResult<T> = Result<T, String>;

fn object_id_from_string(id: &str) -> AppResult<ObjectId> {
    match ObjectId::parse_str(id) {
        Ok(v) => Ok(v),
        Err(e) => Err(String::from("Could not convert string to id.")),
    }
}

#[tauri::command]
pub async fn app_get_tasks_by_day(
    state: tauri::State<'_, AppStateRef>,
    day: String,
) -> AppResult<Vec<TaskRes>> {
    let date = NaiveDateTime::parse_from_str(&day, "%Y-%m-%d %H:%M:%S").unwrap();
    let datetime = date.and_utc();

    // let utc_datetime: DateTime<Utc> = DateTime::from_utc(date, Utc);
    match state.task_service.filter_by_day(datetime).await {
        Ok(value) => Ok(value.into_res()),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
pub async fn app_get_all_tasks(
    state: tauri::State<'_, AppStateRef>,
) -> AppResult<Vec<Vec<TaskRes>>> {
    let result = state.task_service.get_my_tasks().await;

    match result {
        Ok(value) => Ok(value.group_tasks().into_res()),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
pub async fn app_get_task_by_id(
    state: tauri::State<'_, AppStateRef>,
    id: String,
) -> AppResult<Task> {
    let id = object_id_from_string(&id)?;
    let result = state.task_service.find_by_id(&id).await;
    match result {
        Ok(value) => Ok(value),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
pub async fn app_edit_task(
    state: tauri::State<'_, AppStateRef>,
    id: String,
    task: CreateTask,
) -> Result<String, String> {
    let id = object_id_from_string(&id)?;
    let result = state.task_service.update_by_id(&id, task).await;
    match result {
        Ok(value) => Ok(value.upserted_id.unwrap().to_string()),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
pub async fn app_delete_task(
    state: tauri::State<'_, AppStateRef>,
    id: String,
) -> Result<(), String> {
    let id = object_id_from_string(&id)?;
    let result = state.task_service.delete_by_id(&id).await;
    match result {
        Ok(_value) => Ok(()),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
pub async fn app_create_task(
    state: tauri::State<'_, AppStateRef>,
    task: CreateTask,
) -> Result<String, String> {
    let result = state.task_service.create(task).await;
    match result {
        Ok(value) => Ok(value.inserted_id.as_object_id().unwrap().to_hex()),
        Err(e) => Err(e.to_string()),
    }
}
