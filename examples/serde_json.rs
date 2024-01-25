use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

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
    {
        let data = r#"
        {
            "name": "xfy",
            "age": 14,
            "test": null
        }
        "#;

        let unamed: Value = serde_json::from_str(data)?;
        println!("access unamed {:?}", unamed["name"].as_str());
        let named: Test = serde_json::from_str(data)?;
        println!("access named {:?}", named.name);
    }

    {
        let named = Test::default();
        let unamed = json!({
            "name": "xfy",
            "age": 14,
            "test": null
        });
        println!("unamed json string {}", serde_json::to_string(&unamed)?);
        println!("named json string {}", serde_json::to_string(&named)?);
    }

    let data = r#"
        {
            "name": "xfy",
            "age": 14,
            "test": null
        }
        "#;

    println!(
        "deserialize named {:?} unamed {:>}",
        deserialize_named::<Test>(data)?,
        deserialize_named::<Value>(data)?
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
