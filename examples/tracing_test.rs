use std::{thread, time::Duration};

use anyhow::Result;
use tracing::{info, warn};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

fn main() -> Result<()> {
    println!("tracing test");
    // construct a subscriber that prints formatted traces to stdout
    let subscriber = tracing_subscriber::fmt()
        // Use a more compact, abbreviated log format
        .compact()
        // Display source code file paths
        .with_file(true)
        // Display source code line numbers
        .with_line_number(true)
        // Display the thread ID an event was recorded on
        .with_thread_ids(true)
        // Don't display the event's target (module path)
        .with_target(false)
        // Build the subscriber
        .finish();
    // use that subscriber to process traces emitted after this point
    tracing::subscriber::set_global_default(subscriber)?;
    info!("test");
    warn!("test");
    thread::sleep(Duration::from_secs(1));
    Ok(())
}
