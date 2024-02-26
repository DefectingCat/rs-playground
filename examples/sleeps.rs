use anyhow::anyhow;
use std::{thread, time::Duration};
use tokio::{task::JoinHandle, time::sleep};

#[tokio::main]
async fn main() {
    let mut timer = Timer::new(1, || {
        println!("hello world");
    });
    timer.start();

    thread::sleep(Duration::from_secs(3));
    timer.terminate().unwrap();
    thread::sleep(Duration::from_secs(3));
    println!("end");
}

struct Timer {
    duration: Duration,
    job: fn(),
    handler: Option<JoinHandle<()>>,
}

impl Timer {
    pub fn new(time: u64, job: fn()) -> Self {
        let duration = Duration::from_secs(time);
        Self {
            duration,
            job,
            handler: None,
        }
    }

    pub fn start(&mut self) {
        let job = self.job;
        let duration = self.duration;
        let handler = tokio::spawn(async move {
            loop {
                (job)();
                sleep(duration).await;
            }
        });
        self.handler = Some(handler);
    }

    pub fn terminate(&mut self) -> anyhow::Result<()> {
        let handler = self.handler.take();
        handler.ok_or(anyhow!(""))?.abort();
        Ok(())
    }
}
