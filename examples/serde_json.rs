use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Test {
    name: String,
    age: i32,
    test: Option<String>,
}

impl Default for Test {
    fn default() -> Self {
        Self {
            name: "xfy".into(),
            age: 14,
            test: None,
        }
    }
}

fn main() -> Result<()> {
    let data = r#"
        {
            "name": "xfy",
            "age": 14,
            "test": null
        }
        "#;

    println!(
        "deserialize named {:?} unamed",
        deserialize_named::<Test>(data)?
    );

    let data = Test::default();
    println!("serialize named {:?} unamed", serialize_named(&data)?);

    Ok(())
}

fn deserialize_named<'a, T: Deserialize<'a>>(data: &'a str) -> Result<T> {
    let v: T = serde_json::from_str(data)?;
    Ok(v)
}

fn serialize_named<T: Serialize>(data: &T) -> Result<String> {
    let v = serde_json::to_string(data)?;
    Ok(v)
}
