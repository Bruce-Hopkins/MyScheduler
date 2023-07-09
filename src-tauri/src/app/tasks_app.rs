use std::sync::Arc;

use tokio::sync::Mutex;

use crate::{AppState, models::tasks::Time};


#[tauri::command]
async fn get_tasks_by_day(state: tauri::State<'_, Arc<Mutex<AppState>>>, time: Time) -> Result<String, String> {
    todo!()
}

#[tauri::command]
async fn get_all_tasks(state: tauri::State<'_, Arc<Mutex<AppState>>>, time: Time) -> Result<String, String> {
    todo!()
}


#[tauri::command]
async fn get_task_by_id(state: tauri::State<'_, Arc<Mutex<AppState>>>, time: Time) -> Result<String, String> {
    todo!()
}


#[tauri::command]
async fn get_routine_by_id(state: tauri::State<'_, Arc<Mutex<AppState>>>, time: Time) -> Result<String, String> {
    todo!()
}


#[tauri::command]
async fn edit_task(state: tauri::State<'_, Arc<Mutex<AppState>>>, time: Time) -> Result<String, String> {
    todo!()
}


#[tauri::command]
async fn edit_routine(state: tauri::State<'_, Arc<Mutex<AppState>>>, time: Time) -> Result<String, String> {
    todo!()
}


#[tauri::command]
async fn delete_task(state: tauri::State<'_, Arc<Mutex<AppState>>>, time: Time) -> Result<String, String> {
    todo!()
}


#[tauri::command]
async fn delete_routine(state: tauri::State<'_, Arc<Mutex<AppState>>>, time: Time) -> Result<String, String> {
    todo!()
}


