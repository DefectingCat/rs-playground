struct Person {
    name: String,
    age: u32,
}

fn main() {
    // Box
    let point = Box::new((0.625, 0.5));
    let label = format!("{:?}", point);
    println!("Point {}", label);

    // Struct
    let composers = vec![
        Person {
            name: "xfy".to_string(),
            age: 18,
        },
        Person {
            name: "dfy".to_string(),
            age: 18,
        },
        Person {
            name: "Sonetto".to_string(),
            age: 18,
        },
    ];

    composers.iter().for_each(|composer| {
        println!("{}, age {}", composer.name, composer.age);
    });
}
