use std::thread;
use tokio::task::spawn_blocking;

#[tokio::main]
async fn main() {
    let cpus = thread::available_parallelism().unwrap().get();
    let tasks = (0..cpus - 4)
        .map(|i| {
            spawn_blocking(move || {
                dbg!(i);
            })
        })
        .collect::<Vec<_>>();

    for task in tasks {
        task.await.unwrap();
    }
}
