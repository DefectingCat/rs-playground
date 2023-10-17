use std::time::Duration;
use tokio::spawn;
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    let arr = [3, 1, 1, 5];
    let tasks = arr
        .into_iter()
        .map(|i| {
            spawn(async move {
                sleep(Duration::from_millis(i * 100)).await;
                dbg!(i);
            })
        })
        .collect::<Vec<_>>();

    for task in tasks {
        task.await.unwrap();
    }
}
