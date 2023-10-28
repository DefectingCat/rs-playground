use futures::future::try_join_all;

#[tokio::main]
async fn main() {
    let urls = ["https://www.google.com", "https://rua.plus"];
    let sites =
        try_join_all(urls.map(|url| async move { reqwest::get(url).await.unwrap().text().await }))
            .await
            .unwrap();
    sites.iter().for_each(|site| {
        println!("{}", site.len());
    });
}
