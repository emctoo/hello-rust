use std::time::Duration;
use tokio::sync::mpsc::Sender;
use tokio::task::JoinHandle;
use tracing::info;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    info!("hello, world!");

    // capacity = 32?
    let (tx, mut rx) = tokio::sync::mpsc::channel(32);

    let count = 10;
    let mut tasks: Vec<JoinHandle<()>> = Vec::new();
    for i in 0..count {
        let t = tokio::spawn(work(i.to_string(), tx.clone()));
        tasks.push(t);
    }

    while let Some(message) = rx.recv().await {
        info!("got: {:?}", message);
    }
}

async fn work(name: String, tx: Sender<(String, u64)>) {
    info!("launching [{}] ...", name);
    let payload = rand::random::<u64>() % 5;
    tokio::time::sleep(Duration::from_secs(payload)).await;

    info!("sending payload: {}", payload);
    tx.send((name, payload)).await.unwrap();
}
