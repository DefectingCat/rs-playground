const fn five_numbers() -> [i32; 5] {
    let mut numbers = [0i32; 5];

    // for (n, i) in numbers.iter().enumerate() {}
    let mut i = 0_i32;
    while i < 5 {
        numbers[i as usize] = i + 1;
        i += 1;
    }

    numbers
}
const fn numbers<const N: usize>() -> [i32; N] {
    let mut numbers = [0i32; N];

    let mut i = 0;
    while i < N {
        numbers[i] = i as i32 + 1;
        i += 1;
    }

    numbers
}

fn main() {
    let five_numbers = five_numbers();
    println!("{:?}", five_numbers);

    let numbers: [i32; 10] = numbers();
    println!("{:?}", numbers);
}
