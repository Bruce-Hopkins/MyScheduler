use std::{collections::HashMap, future::Future, sync::Arc, time::Duration, vec};

use bson::oid::ObjectId;
use chrono::{DateTime, Days, Timelike, Utc, Local};
use tokio::{sync::Mutex, task::JoinHandle, time::sleep};

use crate::{
    app,
    common::dates::remove_hours_from_date,
    handlers::cron,
    models::{
        routine::Routine,
        tasks::{Task, Time},
    },
    AppState,
};

struct CronjobTask(Task);
impl CronjobTask {
    async fn set_as_overdue(&self, app_state: &AppState) {
        app_state
            .task_service
            .set_task_to_overdue(&self.0.id())
            .await;
    }
}

struct CronjobRoutine(Routine);

impl CronjobRoutine {
    async fn set_as_overdue(&self, app_state: &AppState) {
        // app_state.routine_service.set_task_to_overdue(self.0.id()).await;
    }
}
enum Cronjob {
    Task(CronjobTask),
    Routine(CronjobRoutine),
}

pub struct CronjobHandler(HashMap<String, Vec<Cronjob>>);

impl CronjobHandler {
    pub fn new() -> Self {
        Self(HashMap::new())
    }
    pub fn add_task(&mut self, task: Task) {
        let key = format!("{}-{}", task.end_at().hour(), task.end_at().minute());
        let cronjob_task = CronjobTask(task);
        let cronjob = Cronjob::Task(cronjob_task);
        self.add_cronjob(key, cronjob);
    }

    pub fn add_routine(&mut self, routine: Routine) {
        let key: String = format!("{}-{}", routine.end_at().hour, routine.end_at().minute);
        let cron_routine = CronjobRoutine(routine);
        let cronjob = Cronjob::Routine(cron_routine);
        self.add_cronjob(key, cronjob);
    }

    fn add_cronjob(&mut self, key: String, cronjob: Cronjob) {
        match self.0.get_mut(&key) {
            Some(value) => {
                value.push(cronjob);
            }
            None => {
                self.0.insert(key, vec![cronjob]);
            }
        }
    }

    pub async fn contains_hours(&self, minute: u32, hour: u32) -> bool {
        let key = format!("{hour}-{minute}");
        println!("key is {}", key);
        self.0.contains_key(&key).clone()
    }

    pub async fn run_cronjob(
        &mut self,
        minute: u32,
        hour: u32,
        app_state: &Arc<AppState>,
    ) -> Option<()> {
        let key = format!("{hour}-{minute}");
        let cronjobs = self.0.remove(&key)?;

        let mut continued_cronjobs = vec![];
        for cronjob in cronjobs {
            match cronjob {
                Cronjob::Task(task) => {
                    println!("id {}", task.0.id());

                    if task.0.end_at() < Utc::now() {
                        task.set_as_overdue(&app_state).await;
                    } else {
                        continued_cronjobs.push(Cronjob::Task(task));
                    }
                }
                Cronjob::Routine(routine) => {
                    // let func = &routine.callback;
                    // func().await;
                    // continued_cronjobs.push(Cronjob::Routine(routine));
                }
            }
        }
        // panic!("A task ran!!");

        self.0.insert(key, continued_cronjobs);
        Some(())
    }
}

// pub struct CronTaskHandler(Vec<CronTask>);

// impl CronTaskHandler {
// //     pub async fn add_cron_job(cron_task_handler: AsyncCronTaskHandler, id: &ObjectId, end: DateTime<Utc>, app_state: Arc<Mutex<AppState>>) {
// //         let cron_task_handler= cron_task_handler.clone();
// //         let id = id.clone();
// //         let cron_job = Cronjob::new(end, move || async move {
// //             let mut  cron_task_handler= cron_task_handler.lock().await;

// //             app_state.lock().await.task_service.set_task_to_overdue(&id).await.unwrap();

// //             let iter = (&mut cron_task_handler.0).into_iter();

// //             // Figure out how to remove an element from the list of cronjobs.
// //             let result = iter.enumerate().position(|value| value.1.id == id).unwrap();
// //             cron_task_handler.0.remove(result);
// //         });

// //         cron_task_handler.lock();
// //         // let result = cron_task_handler.lock().await.0.push(CronTask { id: id.clone(), cron_job, task_type: TaskType::Task });
// //     }
// }

// type AsyncCronTaskHandler = Arc<Mutex<CronTaskHandler>>;

// impl CronTaskHandler {
//     pub fn new() -> Self {
//         Self(vec![])
//     }
// }

pub fn milliseconds_between_dates(start: DateTime<Utc>, end: DateTime<Utc>) -> i64 {
    let duration = end.signed_duration_since(start);
    duration.num_milliseconds()
}

pub async fn add_cronjob_tasks(cron_handler: &Arc<Mutex<CronjobHandler>>, tasks: Vec<Task>) {
    for task in tasks {
        cron_handler.lock().await.add_task(task)
    }
}

pub async fn add_cronjob_routines(cron_handler: &Arc<Mutex<CronjobHandler>>, tasks: Vec<Routine>) {
    for task in tasks {
        cron_handler.lock().await.add_routine(task)
    }
}

pub async fn run_cronjobs(cron_handler: &Arc<Mutex<CronjobHandler>>, app_state: &Arc<AppState>) {
    let now = Local::now();
    let mut cron_handler = cron_handler.lock().await;

    println!("Running cron job task");
    if cron_handler.contains_hours(now.minute(), now.hour()).await {
        cron_handler
            .run_cronjob(now.minute(), now.hour(), app_state)
            .await;
    }
}
