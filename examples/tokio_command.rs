use std::process::Stdio;

use anyhow::Result;
use tokio::io::{AsyncReadExt, AsyncWriteExt, BufReader};
use tokio::process::Command;

#[tokio::main()]
async fn main() -> Result<()> {
    let output = Command::new("ls").args(["-al"]).output().await?.stdout;
    let stdout = String::from_utf8_lossy(&output);
    println!("{}", stdout);

    let mut child = Command::new("apate")
        .args(["encrypt", "-p", "123"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;

    let mut child_in = child.stdin.take().unwrap();
    let input = "xfy";
    child_in.write_all(input.as_bytes()).await.unwrap();

    child.wait().await.unwrap();

    let child_out = child.stdout.take().unwrap();
    let mut child_out = BufReader::new(child_out);
    let mut buffer = vec![];
    let _ = child_out.read_to_end(&mut buffer).await.unwrap();
    dbg!(buffer);

    Ok(())
}
