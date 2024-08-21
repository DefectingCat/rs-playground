use anyhow::{anyhow, Error, Ok};
use std::{thread, time::Duration};

fn main() {
    let handler = thread::spawn(|| {
        println!("hello world thread one");
        let test: Option<String> = None;
        test.unwrap();
        anyhow!("")
    });

    thread::sleep(Duration::from_secs(2));
    println!("hello world form main thread");
    let _ = handler.join().map_err(|err| {
        println!("child thread exit failed {:?}", err);
    });
}
