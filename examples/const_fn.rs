const fn five_numbers() -> [u32; 5] {
    let mut numbers = [0u32; 5];

    let mut i = 0;
    while i < 5 {
        numbers[i] = i as u32 + 1;
        i += 1;
    }

    numbers
}

const fn numbers<const N: usize>() -> [u32; N] {
    let mut numbers = [0u32; N];

    let mut i = 0;
    while i < N {
        numbers[i] = i as u32 + 1;
        i += 1;
    }

    numbers
}

const fn len(strs: &[&str]) -> usize {
    let mut result = 0;
    let mut remaining = strs;

    // get first element in array, calculate its length
    // then use slice deconstruction to update remaining
    // until empty
    while let [current, tail @ ..] = remaining {
        result += current.len();
        remaining = tail;
    }

    result
}

fn main() {
    let five_numbers = five_numbers();
    println!("five numerbs {:?}", five_numbers);

    let numbers: [u32; 10] = numbers();
    println!("generic numbers {:?}", numbers);

    let strs = ["xfy", "dfy", "wudixiaofeiyang"];
    let str_len = len(&strs);
    println!("strs length {}", str_len);
}
