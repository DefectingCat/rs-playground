use std::{
    ops::Sub,
    sync::{Arc, Mutex},
    thread,
    time::Instant,
};

const N_TIMES: u64 = 10_000_000;

fn add_n_times(n: u64, counter: Arc<Mutex<u64>>) {
    let mut counter = counter.lock().unwrap();
    (0..n).for_each(|_| {
        *counter += 1;
    })
}

fn main() {
    let s = Instant::now();
    let counter = Arc::new(Mutex::new(0u64));
    let cpus = thread::available_parallelism().unwrap().get();
    let mut handles = Vec::with_capacity(cpus);

    (0..cpus).for_each(|_| {
        let counter = counter.clone();
        let h = thread::spawn(move || add_n_times(N_TIMES, counter));
        handles.push(h);
    });

    for handle in handles {
        handle.join().unwrap();
    }
    let counter = counter.lock().unwrap();
    println!("{}", *counter);
    println!("{:?}", Instant::now().sub(s));
}
