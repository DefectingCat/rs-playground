use std::{
    ops::Sub,
    sync::atomic::{AtomicU64, Ordering},
    thread,
    time::Instant,
};

const N_TIMES: u64 = 10_000_000;

static R: AtomicU64 = AtomicU64::new(0);

fn add_n_times(n: u64) {
    (0..n).for_each(|_| {
        R.fetch_add(1, Ordering::Relaxed);
    })
}

fn main() {
    let s = Instant::now();
    let cpus = thread::available_parallelism().unwrap().get();

    let mut handles = Vec::with_capacity(cpus);
    (0..cpus).for_each(|_| {
        let h = thread::spawn(move || add_n_times(N_TIMES));
        handles.push(h);
    });

    for handle in handles {
        handle.join().unwrap();
    }

    println!("{}", R.load(Ordering::Relaxed));
    println!("{:?}", Instant::now().sub(s));
}
