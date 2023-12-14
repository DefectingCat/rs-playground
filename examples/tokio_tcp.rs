use log::info;
use tokio::net::TcpListener;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();

    let listener = TcpListener::bind("127.0.0.1:6969").await?;

    loop {
        let (socket, _) = listener.accept().await?;
        let addr = socket.peer_addr()?;
        info!("addr {}", addr);
    }
}
