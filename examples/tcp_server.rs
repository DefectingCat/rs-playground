use log::info;
use std::net::TcpListener;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();

    let listener = TcpListener::bind("127.0.0.1:6969")?;

    listener.incoming().try_for_each(|stream| {
        let s = stream?;
        let addr = s.peer_addr()?;
        info!("client incoming {}", addr);
        anyhow::Ok(())
    })?;

    Ok(())
}
