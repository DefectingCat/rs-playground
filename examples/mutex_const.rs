use std::{sync::Mutex, thread};

static H: Mutex<u32> = Mutex::new(0);

fn main() {
    let binding = &H;

    let handle = thread::spawn(move || {
        let binding = &H;
        let mut h = binding.lock().unwrap();
        *h += 40;
        println!("Child thread: {}", *h);
    });

    handle.join().unwrap();

    let mut h = binding.lock().unwrap();
    *h += 2;
    println!("{}", h);
}
