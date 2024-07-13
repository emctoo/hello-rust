use tokio::time::{sleep, Duration};
use tracing::{debug, error, info, warn};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    warn!("hi");
    error!("some more errors");
    debug!("debug output");

    let sleep = sleep(Duration::from_secs(3));
    // pin!(sleep);
    sleep.await;
    info!("done");
    // select!
}

