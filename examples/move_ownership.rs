fn main() {
    let arr = vec!["udon", "ramen", "soba"];
    let trr = arr.clone();
    let urr = arr.clone();

    println!("Address of arr {:?}", &arr as *const Vec<&str>);
    println!("Address of trr {:?}", &trr as *const Vec<&str>);
    println!("Address of urr {:?}", &urr as *const Vec<&str>);

    let x = vec![1, 2, 3];
    if c {
        f(x);
    } else {
        g(x);
    }
    h(x);

    let x = vec![1, 2, 3];
    while f() {
        g(x);
    }

    let v = vec!["Hello".to_string(), "World".to_string()];
    let second = v[1];
}
