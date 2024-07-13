use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Waker};
use std::time::Duration;
use tokio::select;
use tracing::info;

// async book, Chapter 2, Section 2
// https://rust-lang.github.io/async-book/02_execution/03_wakeups.html

#[tokio::main]
async fn main() {
    // tracing_subscriber::fmt::fmt()
    //     .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
    //     .init();
    console_subscriber::init(); // run `tokio-console` in a different terminal, run this with
                                // RUST_LOG=wakeup=debug

    let timer = TimerFuture::new(Duration::from_secs(5));
    info!("- timer created");

    // 考虑下面折中写法，不使用 select! 宏，sleep 几秒钟，然后再 await timer
    // 会 poll 一次，pending
    // 然后会等到 thread 中的 sleep 结束了，再 poll
    // info!("- sleeping");
    // tokio::time::sleep(Duration::from_secs(2)).await;
    // info!("- sleep is done");
    // timer.await;

    // interval 时，每一秒都会 yield 给 timer，看看 timer 的 poll 方法如何处理
    // 作为对比，下面的会 poll 多次
    let mut interval = tokio::time::interval(Duration::from_secs(1));

    info!("looping ...");
    loop {
        select! {
            _ = timer.clone() => {
                info!("- timer expired");
                break;
            }
            t = interval.tick() => {
                info!("**** tick {:?} ****", t);
            }
        }
    }
}

struct SharedState {
    completed: bool,
    waker: Option<Waker>,
}

#[derive(Clone)]
struct TimerFuture {
    shared_state: Arc<Mutex<SharedState>>,
}

impl TimerFuture {
    pub fn new(duration: Duration) -> Self {
        let shared_state = Arc::new(Mutex::new(SharedState {
            completed: false,
            waker: None,
        }));

        let shared_state_clone = shared_state.clone();
        tokio::spawn(async move {
            info!("t / launched");

            info!("t / sleeping ...");
            tokio::time::sleep(duration).await;

            info!("t / update shared state ...");
            let mut shared_state = shared_state_clone.lock().unwrap();
            shared_state.completed = true;
            if let Some(waker) = shared_state.waker.take() {
                info!("t / wake up the waker ...");
                waker.wake();
            }
            info!("t / done");
        });

        TimerFuture { shared_state }
    }
}

impl Future for TimerFuture {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        info!("P / ==== polling ...");
        let mut shared_state = self.shared_state.lock().unwrap();

        if shared_state.completed {
            info!("P / completed");
            Poll::Ready(())
        } else {
            info!("P / not completed yet");
            shared_state.waker = Some(cx.waker().clone());

            info!(
                "P / cloned the waker {:?}, returning pending ...",
                cx.waker()
            );
            Poll::Pending
        }
    }
}
