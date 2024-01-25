use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Test {
    test: Option<String>,
}

fn main() -> Result<()> {
    let data = r#"
    {
        "test": null
    }
    "#;

    let v: Test = serde_json::from_str(data)?;
    dbg!(v);

    Ok(())
}
