use std::{future::Future, time::Duration, sync::Arc, vec, collections::HashMap};

use bson::oid::ObjectId;
use chrono::{DateTime, Utc, Days, Timelike};
use tokio::{time::sleep, sync::Mutex, task::JoinHandle};


use crate::{common::dates::remove_hours_from_date, AppState, app, models::tasks::Time, handlers::cron};


struct CronjobTask<T, F>
    where F: FnOnce() -> T + std::marker::Send + 'static,
    T: Future<Output = ()> + std::marker::Send + 'static{
        callback: F,
        end_date: DateTime<Utc>
}


struct CronjobRoutine<T, F>
    where F: Fn() -> T + std::marker::Send + 'static,
    T: Future<Output = ()> + std::marker::Send + 'static{
        callback: F,
        end_date: Time
}

enum Cronjob <T, F>
where F: Fn() -> T + std::marker::Send + 'static,
T: Future<Output = ()> + std::marker::Send + 'static {
    Task(CronjobTask<T, F>),
    Routine(CronjobRoutine<T, F>)
}



pub struct CronjobHandler<F, T>(HashMap<String, Vec<Cronjob<T, F>>>) 
    where F: Fn() -> T + std::marker::Send + 'static,
    T: Future<Output = ()> + std::marker::Send + 'static;

impl <F, T> CronjobHandler<F, T> 
    where F: Fn() -> T + std::marker::Send + 'static,
    T: Future<Output = ()> + std::marker::Send + 'static
{
    pub fn add_task(&mut self, end_date: DateTime<Utc>, callback: F) -> () {
        let key = format!("{}-{}", end_date.hour(), end_date.minute());
        let cronjob_task = CronjobTask {
            callback,
            end_date
        };
        let cronjob = Cronjob::Task(cronjob_task);
        self.add_cronjob(key, cronjob);
    }

    pub fn add_routine(&mut self, end_date: Time, callback: F) -> () {
        let key = format!("{}-{}", end_date.hour, end_date.minute);
        let cronjob_task = CronjobRoutine {
            callback,
            end_date
        };
        let cronjob = Cronjob::Routine(cronjob_task);
        self.add_cronjob(key, cronjob);
    }

    fn add_cronjob(&mut self, key: String, cronjob: Cronjob<T,F>) {
        match self.0.get_mut(&key) {
            Some(value) => {
                value.push(cronjob);
            },
            None => {
                self.0.insert(key, vec![cronjob]);
            },
        }
    }

    pub async fn run_cronjob(&mut self, minute: i32, hour: i32) -> Option<()> {
        let key = format!("{minute}-{hour}");
        let cronjobs = self.0.remove(&key)?;

        let mut continued_cronjobs = vec![];
        for cronjob in cronjobs {
            match cronjob {
                Cronjob::Task(task) => {
                    if task.end_date < Utc::now() {
                        let func = task.callback;
                        func().await;
                    }
                    else {
                        continued_cronjobs.push(Cronjob::Task(task));
                    }
                },
                Cronjob::Routine(routine) => {
                    let func = &routine.callback;
                    func().await;
                    continued_cronjobs.push(Cronjob::Routine(routine));
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