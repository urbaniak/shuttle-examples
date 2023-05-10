use std::{future::Future, str::FromStr};

use cron::Schedule;
use shuttle_crontab::{CronJob, CronService};
use shuttle_runtime::tracing::info;
use shuttle_service::Error;

// "Run every 2 seconds"
const SCHEDULE: &str = "*/2 * * * * *";

// The function that will be run.
async fn my_job() {
    let now = chrono::offset::Utc::now();
    info!("It is {}", now.format("%Y-%m-%d %H:%M:%S"));
}

#[shuttle_runtime::main]
async fn init() -> Result<CronService<impl Future<Output = ()>>, Error> {
    let schedule = Schedule::from_str(SCHEDULE).unwrap();
    Ok(CronService {
        jobs: vec![
            CronJob {
                schedule: schedule.clone(),
                job: my_job,
            },
            CronJob {
                schedule: Schedule::from_str("*/10 * * * * *").unwrap(),
                job: my_job,
            },
        ],
    })
}
