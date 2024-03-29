use rand::random;
use std::fmt::Display;

fn main() {
    let mut strings = Vec::new();
    for _ in 0..10 {
        if random() {
            let string = leak_static();
            strings.push(string)
        }
    }

    strings.iter().for_each(test_borrow);
    strings.into_iter().for_each(test_static);
    // strings.into_iter().for_each(drop_static);

    println!("end of the program");
}

fn leak_static() -> &'static str {
    let rand_string = rand::random::<u64>().to_string();
    Box::leak(rand_string.into_boxed_str())
}

fn test_borrow<T: Display + ?Sized>(rand_string: &T) {
    println!("random leak string {}", rand_string);
}

fn test_static<T: Display + ?Sized>(rand_string: &'static T) {
    println!("random leak string {}", rand_string);
}

fn drop_static<T: 'static>(target: T) {
    std::mem::drop(target);
}
