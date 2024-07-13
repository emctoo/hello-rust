use rand::random;
use std::fmt::Display;
use tokio::sync::broadcast;
use tokio::sync::mpsc;
use tokio::task::JoinHandle;
use tracing::info;

/// broadcast: 就是 mpmc

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::fmt()
        .with_env_filter("mpmc=debug")
        .init();

    info!("hello, world!");

    // distribute the workload
    let (tx, _) = broadcast::channel(16);

    let count = 10;

    // use this to collect the result
    let (result_tx, mut result_rx) = tokio::sync::mpsc::channel::<(usize, i8)>(count);

    let mut futs: Vec<JoinHandle<()>> = Vec::new();
    for idx in 1..count {
        futs.push(tokio::spawn(worker(
            idx,
            uuid::Uuid::new_v4().to_string(),
            tx.subscribe(),
            result_tx.clone(),
        )));
    }

    let payload_total = 10;
    let mut payloads = (0..payload_total)
        // .map(|_| Some(random::<i8>()))
        .map(|i| Some(i))
        .collect::<Vec<_>>();
    payloads.push(None);
    payloads.into_iter().for_each(|payload| {
        tx.send(payload).unwrap();
    });

    // a different way
    // futures::future::join_all(futs).await;

    while let Some((task_idx, payload)) = result_rx.recv().await {
        info!("++++ from task {}: {}", task_idx, payload);
    }
    info!("done");
}

async fn worker<T: Clone + Display>(
    task_idx: usize,
    name: String,
    mut rx: broadcast::Receiver<Option<T>>,
    result_tx: mpsc::Sender<(usize, i8)>,
) {
    while let Ok(payload) = rx.recv().await {
        match payload {
            Some(val) => {
                // TODO: sum for generic type `T`
                info!("{} / task {}: {}", task_idx, name, val);
            }
            None => {
                info!("{} / task {}, received None, exit now ...", task_idx, name);
                result_tx.send((task_idx, random::<i8>())).await.unwrap();
                break;
            }
        }
    }
    info!("{} / done", name);
}
