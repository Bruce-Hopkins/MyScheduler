use std::sync::Arc;

use bson::DateTime;
use chrono::{Utc, NaiveDate, NaiveDateTime};
use tokio::sync::Mutex;

use crate::{AppState, models::tasks::{Time, Task}, common::errors::DBResult};

type AppStateRef = Arc<AppState>;

type AppResult<T> = Result<T, String>;


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
async fn get_all_tasks(state: tauri::State<'_, AppStateRef>, time: Time) -> Result<String, String> {
    todo!()
}


#[tauri::command]
async fn get_task_by_id(state: tauri::State<'_, AppStateRef>, time: Time) -> Result<String, String> {
    todo!()
}

#[tauri::command]
async fn edit_task(state: tauri::State<'_, AppStateRef>, time: Time) -> Result<String, String> {
    todo!()
}

#[tauri::command]
async fn delete_task(state: tauri::State<'_, AppStateRef>, time: Time) -> Result<String, String> {
    todo!()
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


