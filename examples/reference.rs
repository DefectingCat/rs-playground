#[allow(dead_code)]
struct Person<'a, 'b> {
    pub name: &'a str,
    pub age: &'b u32,
}

fn main() {
    let name = String::from("xfy");
    let n;
    {
        let age = 18u32;
        let xfy = Person {
            name: &name,
            age: &age,
        };
        n = xfy.name;
    }
    println!("{}", n);
}
