use log::{debug, error, info, warn};
use tokio::time::{Duration, sleep};

#[tokio::main]
async fn main() {
    env_logger::builder().parse_filters("info").init();

    warn!("hi");
    error!("some more errors");
    debug!("debug output");

    let sleep = sleep(Duration::from_secs(3));
    // pin!(sleep);
    sleep.await;
    info!("done");
    // select!
}