use std::sync::Arc;

use bson::{DateTime, oid::ObjectId};
use chrono::{Utc, NaiveDate, NaiveDateTime};
use tokio::sync::Mutex;

use crate::{AppState, models::tasks::{Time, Task, CreateTask}, common::errors::DBResult};

type AppStateRef = Arc<AppState>;

type AppResult<T> = Result<T, String>;

fn object_id_from_string(id: &str) -> AppResult<ObjectId>{
    match ObjectId::parse_str(id) {
        Ok(v) => Ok(v),
        Err(e) => Err(String::from("Could not convert string to id."))
    }
}


#[tauri::command]
async fn get_tasks_by_day(state: tauri::State<'_, AppStateRef>, day: String) -> AppResult<Vec<Task>>{
    let date = NaiveDateTime::parse_from_str(&day, "%Y-%m-%d %H:%M:%S").unwrap();
    let datetime = date.and_utc();

    // let utc_datetime: DateTime<Utc> = DateTime::from_utc(date, Utc);
    match state.task_service.filter_by_day(datetime).await {
        Ok(value) => Ok(value),
        Err(e) => Err(e.to_string())
    }
}

#[tauri::command]
async fn get_all_tasks(state: tauri::State<'_, AppStateRef>) -> AppResult<Vec<Task>>{
    let result = state.task_service.get_my_tasks().await;
    match result {
        Ok(value) => Ok(value),
        Err(e) => Err(e.to_string())
    }
}


#[tauri::command]
async fn get_task_by_id(state: tauri::State<'_, AppStateRef>, id: String) -> AppResult<Task> {
    let id = object_id_from_string(&id)?;
    let result = state.task_service.find_by_id(&id).await;
    match result {
        Ok(value) => Ok(value),
        Err(e) => Err(e.to_string())
    }
}

#[tauri::command]
async fn edit_task(state: tauri::State<'_, AppStateRef>, id: String, task: CreateTask) -> Result<String, String> {
    let id = object_id_from_string(&id)?;
    let result = state.task_service.update_by_id(&id, task).await;
    match result {
        Ok(value) => Ok(value.upserted_id.unwrap().to_string()),
        Err(e) => Err(e.to_string())
    }

}

#[tauri::command]
async fn delete_task(state: tauri::State<'_, AppStateRef>, id: String) -> Result<(), String> {
    let id = object_id_from_string(&id)?;
    let result = state.task_service.delete_by_id(&id).await;
    match result {
        Ok(_value) => Ok(()),
        Err(e) => Err(e.to_string())
    }
}

#[tauri::command]
async fn create_task(state: tauri::State<'_, AppStateRef>, task: CreateTask) -> Result<String, String> {
    let result = state.task_service.create(task).await;
    match result {
        Ok(value) => Ok(value.inserted_id.to_string()),
        Err(e) => Err(e.to_string())
    }
}




// #[tauri::command]
// async fn get_routine_by_id(state: tauri::State<'_, Arc<Mutex<AppState>>>, time: Time) -> Result<String, String> {
//     todo!()
// }



// #[tauri::command]
// async fn edit_routine(state: tauri::State<'_, Arc<Mutex<AppState>>>, time: Time) -> Result<String, String> {
//     todo!()
// }




// #[tauri::command]
// async fn delete_routine(state: tauri::State<'_, Arc<Mutex<AppState>>>, time: Time) -> Result<String, String> {
//     todo!()
// }


