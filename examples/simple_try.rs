fn main() {
    let arr = [1, 2, 3];

    // arr.iter().fold(init, f);

    let mut total = 0;
    simple_fold(arr.iter_mut(), total, |prev, cur| {
        dbg!(&prev, &cur);
        // prev += cur;
        prev
    })
}

fn simple_fold<A, T>(
    iter: impl Iterator<Item = T>,
    mut init: A,
    mut f: impl FnMut(A, T) -> A,
) -> A {
    for x in iter {
        init = f(init, x)
    }
    init
}
