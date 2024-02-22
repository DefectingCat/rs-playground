use futures::Future;

#[tokio::main]
async fn main() {
    let num = test(calc).await;
    println!("{num}");
}

async fn calc() -> i32 {
    40 + 2
}

async fn test<F, Fut>(f: F) -> i32
where
    F: FnOnce() -> Fut,
    Fut: Future<Output = i32>,
{
    f().await
}
