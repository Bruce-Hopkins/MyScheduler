use std::{future::Future, time::Duration, sync::Arc, vec, collections::HashMap};

use bson::oid::ObjectId;
use chrono::{DateTime, Utc, Days, Timelike};
use tokio::{time::sleep, sync::Mutex, task::JoinHandle};


use crate::{common::dates::remove_hours_from_date, AppState, app, models::{tasks::{Time, Task}, routine::Routine}, handlers::cron};


struct CronjobTask(Task);
impl CronjobTask {
    async fn set_as_overdue(&self, app_state: &AppState) {
        app_state.task_service.set_task_to_overdue(self.0.id()).await;
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
    Routine(CronjobRoutine)
}



pub struct CronjobHandler(HashMap<String, Vec<Cronjob>>);

impl CronjobHandler {
    pub fn new() -> Self {
        todo!()
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
            },
            None => {
                self.0.insert(key, vec![cronjob]);
            },
        }
    }

    pub async fn run_cronjob(&mut self, minute: u32, hour: u32, app_state: &Arc<AppState>) -> Option<()> {
        let key = format!("{minute}-{hour}");
        let cronjobs = self.0.remove(&key)?;

        let mut continued_cronjobs = vec![];
        for cronjob in cronjobs {
            match cronjob {
                Cronjob::Task(task) => {
                    if task.0.end_at() < Utc::now() {
                        task.set_as_overdue(&app_state).await;
                    }
                    else {
                        continued_cronjobs.push(Cronjob::Task(task));
                    }
                },
                Cronjob::Routine(routine) => {
                    // let func = &routine.callback;
                    // func().await;
                    // continued_cronjobs.push(Cronjob::Routine(routine));
                },
            }
        } 
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

pub fn milliseconds_between_dates(start:DateTime<Utc>,  end: DateTime<Utc>) -> i64 {
    let duration = end.signed_duration_since(start);
    duration.num_milliseconds()
}

pub async fn crontask<F, T>(end: DateTime<Utc>, callback: F) 
    where F: FnOnce() -> T,
    T: Future<Output = T> + 'static {

}

pub async fn end_of_day_scheduler(today: DateTime<Utc>, app_state: Arc<Mutex<AppState>>) {
    let tomorrow = today.checked_add_days(Days::new(1)).unwrap();
    let tomorrow = remove_hours_from_date(tomorrow).unwrap();
    let miliseconds_left_in_day = milliseconds_between_dates(today, tomorrow);

    let handle = tokio::spawn(async {
        // Do some async work
        "return value"
    });
    sleep(Duration::from_millis(miliseconds_left_in_day as u64)).await;

    // Find the tasks and routines in the app.

}