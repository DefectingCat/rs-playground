use anyhow::Result;
use tokio::{
    sync::oneshot,
    time::{sleep, Duration},
};

#[tokio::main]
async fn main() -> Result<()> {
    let (tx1, rx1) = oneshot::channel();

    tokio::spawn(async {
        sleep(Duration::from_millis(99)).await;
        let _ = tx1.send("one");
    });

    tokio::select! {
        val = rx1 => {
            println!("rx1 completed first with {:?}", val?);
        }
        _ = sleep(Duration::from_millis(100)) => {
            println!("sleep from 100 milliseconds");
        }
    }

    Ok(())
}
