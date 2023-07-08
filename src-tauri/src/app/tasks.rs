
#[tauri::command]
async fn filter_by_day(state: tauri::State<'_, Arc<Mutex<AppState>>>, time: Time) -> Result<String, String> {
    todo!()
}