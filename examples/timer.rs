use anyhow::Result;
use std::time::Duration;
use tokio_cron_scheduler::{Job, JobScheduler};

#[tokio::main]
async fn main() -> Result<()> {
    let sched = JobScheduler::new().await?;
    sched
        .add(Job::new("1/10 * * * * *", |uuid, _| {
            println!("every 10 seconds, uuid {}", uuid);
        })?)
        .await?;

    sched.start().await?;

    tokio::time::sleep(Duration::from_secs(100)).await;
    Ok(())
}
