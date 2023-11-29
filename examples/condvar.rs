use std::{
    sync::{Arc, Condvar, Mutex},
    thread,
    time::Duration,
};

fn main() {
    let _cpus = thread::available_parallelism().unwrap().get();

    let flag = Arc::new(Mutex::new(false));
    let cond = Arc::new(Condvar::new());

    let cflag = flag.clone();
    let ccond = cond.clone();
    let handler = thread::spawn(move || {
        let mut lock = cflag.lock().unwrap();
        let mut counter = 0;

        while counter < 3 {
            while !*lock {
                lock = ccond.wait(lock).unwrap();
            }

            *lock = false;
            counter += 1;
            println!("Child counter: {}", counter);
        }
    });

    let mut counter = 0;
    while counter < 3 {
        thread::sleep(Duration::from_millis(1000));
        *flag.lock().unwrap() = true;
        counter += 1;
        println!("Parent counter: {}", counter);
        cond.notify_one();
    }

    handler.join().unwrap();
}
