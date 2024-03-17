fn main() -> anyhow::Result<()> {
    let arr = [String::from("1"), String::from("2"), String::from("3")];

    arr.into_iter().for_each(|item| {
        println!("{}", item);
    });
    // println!("{:?}", arr);

    Ok(())
}
