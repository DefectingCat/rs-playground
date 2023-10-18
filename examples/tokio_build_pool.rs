use anyhow::Result;
use std::thread;
use tokio::runtime;

fn main() -> Result<()> {
    let cpus = thread::available_parallelism()?.get();

    for i in 1..=cpus {
        thread::spawn(move || {
            dbg!(i);

            let rt = runtime::Builder::new_current_thread().build().unwrap();
            drop(rt);
            dbg!(i);

            // rt.block_on(async {
            //     tokio::spawn(async move {
            //         dbg!("hello");
            //         dbg!(i);
            //     })
            //     .await
            //     .unwrap();
            // });
        });
    }

    Ok(())
}
