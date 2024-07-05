use std::str::FromStr;
use actix_web::web;
use chrono::Utc;
use cron::Schedule;
use crate::AppState;
use crate::schedule::demo_schedule::demo_schedule;

mod demo_schedule;

pub async fn run_demo_schedule(app_state: web::Data<AppState<'static>>) {
    actix_rt::spawn(async move {
        let expression = "*  */30  *  *  *  *  *";
        let schedule = Schedule::from_str(expression).unwrap();

        loop {
            let now = Utc::now();
            let next_schedule = schedule.upcoming(Utc).take(1).next();

            if let Some(next) = next_schedule {
                let until_next = next - now;
                actix_rt::time::sleep(until_next.to_std().unwrap()).await;

                demo_schedule(app_state.clone()).await.unwrap();
            }
        }
    });
}
