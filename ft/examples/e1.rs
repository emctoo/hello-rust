#![allow(unused)]
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::thread;
use futures::future;
use future::FutureExt;
// use std::time::Duration;
use tokio::time::Duration;

// sleep 返回的 Sleep 是 Future
use tokio::time::{sleep, Sleep};

use log::info;

// #[tokio::main(flavor = "current_thread")]
#[tokio::main]
async fn main() {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    // let rt = tokio::runtime::Builder::new_current_thread()
    //     .build()
    //     .unwrap();
    let s = sleep(Duration::from_secs(3));
    info!("s: {:?}", s);
    let result = s.await;
    info!("result: {:?}", result);

    {
        info!("create a ft");
        let fut = MyFuture::new(3);

        info!("await ...");
        fut.await;
    }
    // rt.block_on(fut);
    info!("done");
}

struct MyFuture {
    counter: u8,
    sleep: Pin<Box<Sleep>>,
}

impl MyFuture {
    fn new(counter: u8) -> Self {
        MyFuture {
            counter,
            sleep: Box::pin(sleep(Duration::from_secs(1))), // just a future, stored
        }
    }
}

impl Future for MyFuture {
    type Output = ();
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // sleep(Duration::from_secs(2));
        // Poll::Ready(())

        info!("poll ... {}", self.counter);

        // 就地 wake
        // cx.waker().wake_by_ref();

        if self.counter == 0 {
            info!("ready now");
            return Poll::Ready(());
        }

        // 在另外一个线程中调用 wake
        // {
        //     let waker = cx.waker().clone();
        //     thread::spawn(move || {
        //         std::thread::sleep(Duration::from_secs(2));
        //         info!("wake up in thread ...");
        //         waker.wake();
        //     });
        // }

        // let sleep = Pin::new(&mut self.sleep);
        info!("sleep: {:?}", self.sleep);

        // self.sleep.as_mut().poll(cx)

        // let t = self.sleep.as_mut();
        // t.poll(cx)

        self.sleep.poll_unpin(cx)

        // sleep.poll(cx)
        // info!("slept");

        // self.counter -= 1;
        // info!("counter: {}, returning pending ...", self.counter);
        // Poll::Pending
    }
}
