#[tokio::main]
async fn main() {
    let test = || async { println!("async closure") };
    let atest = async || println!("async closure");
}
