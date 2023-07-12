
// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod models;
pub mod handlers;
pub mod services;
pub mod tests;
pub mod common;
pub mod app;


use handlers::cron::CronTaskHandler;
use models::tasks::{Task, Time};
use mongodb::{Collection, Database, options::ClientOptions, Client};
use services::tasks_service::TasksService;
use tokio::sync::Mutex;
use std::{env, sync::Arc};
pub struct EnvConfig {
    // App
    pub app_env: String,
    pub port: String,

    // Database
    pub db_host: String,
    pub db_port: String,
    pub user_credentials: String,

    // Frontend
    pub frontend_host: String,

    // Sendgrid key
    pub sendgrid_api_key: String,

    // JWT Secret
    pub jwt_secret: String,
}

impl EnvConfig {
    pub fn new() -> Self {
        // App
        let app_env = match env::var("DEV_ENV") {
            Ok(app_env) => app_env,
            Err(_e) => String::from("test"),
        };

        let port = match env::var("PORT") {
            Ok(app_env) => app_env,
            Err(_e) => String::from("8080"),
        };

        // Database
        let db_host = match env::var("DB_HOST") {
            Ok(app_env) => app_env,
            Err(_e) => String::from("localhost"),
        };
        let db_port = match env::var("DB_PORT") {
            Ok(app_env) => app_env,
            Err(_e) => String::from("27017"),
        };

        let user_credentials = if let (Ok(username), Ok(password)) =
            (env::var("DB_USERNAME"), env::var("DB_PASSWORD"))
        {
            let res = format!("{username}:{password}@");
            res
        } else {
            String::from("")
        };

        // Frontend
        let frontend_host = match env::var("FRONTEND_URL") {
            Ok(app_env) => app_env,
            Err(_e) => String::from("http://localhost:3000"),
        };

        // Sendgrid API key
        let sendgrid_api_key = match env::var("SENDGRID_API_KEY") {
            Ok(api_key) => api_key,
            Err(_e) => String::from(""),
        };

        let jwt_secret = match env::var("JWT_SECRET") {
            Ok(key) => key,
            Err(_e) => String::from("secret"),
        };

        Self {
            app_env,
            port,
            db_host,
            db_port,
            user_credentials,
            frontend_host,
            sendgrid_api_key,
            jwt_secret,
        }
    }

    pub fn reset_password_url(&self, reset_token: &str) -> String {
        format!("{}/reset/{}", &self.frontend_host, reset_token)
    }
}

pub struct AppState {
    task_service: TasksService,
    test: String,
}
impl AppState {
    /**
     * It can return a collection or a collection that has been dropped.
     * Preconditions: Set the db, name of the collection, and whether the db should be dropped.
     * Postconditons: If the variable `should_drop_col` is true, then it will drop the col If not, it won't.
     * It will always return the collection
    */
    pub async fn start_collections<T>(db: &Database, name: &str) -> Collection<T> {
        db.collection(name)
    }

    async fn new(db: &Database) -> Self {
        let task_col = Self::start_collections(&db, "tasks").await;
        let task_service = TasksService::new(task_col);
        Self {
            task_service,
            test: "test".to_string()
        }
    }

    fn into_arc(self) -> Arc<Mutex<Self>> {
        let mutex = Mutex::new(self);
        Arc::new(mutex)
    }
}



#[tauri::command]
async fn my_custom_command(state: tauri::State<'_, Arc<Mutex<AppState>>>, time: Time) -> Result<String, String> {
    let test = &state.lock().await.test;
    Ok(format!("{}:{}. Test:{}", time.hour, time.minute, test))
}

async fn startup_script() {
    
}

#[tokio::main]
async fn main() {
  
    let db = init_db().await;
    let state = AppState::new(&db).await;
    let cron_task_handler = CronTaskHandler::new();
    let cron_task_handler = Arc::new(Mutex::new(cron_task_handler));

    startup_script().await;

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![my_custom_command]) 
        .manage(state.into_arc())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}



pub async fn init_db() -> Database {
    let db_name = String::from("time-managing-app");

    // Get the env variables
    let config = EnvConfig::new();
    let url = format!(
        "mongodb://{}{}:{}",
        config.user_credentials, config.db_host, config.db_port
    );
    println!("{}", url);

    // Setup the client
    let client_options = ClientOptions::parse(&url)
        .await
        .expect("Could not connect to database");

    let client = Client::with_options(client_options).expect("Could not get database");
    client.database(&db_name.as_str())
}
