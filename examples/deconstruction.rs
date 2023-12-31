fn main() {
    let arr = [1, 2, 3, 4, 5, 6, 7, 8];

    let [first, second, tail @ ..] = arr;

    dbg!(first, second, tail);
}
