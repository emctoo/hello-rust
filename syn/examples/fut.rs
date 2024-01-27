use std::future::Future;
use std::process::Output;
use std::time::Duration;

use futures::channel::mpsc;
use futures::executor;
//standard executors to provide a context for futures and streams
use futures::executor::ThreadPool;
use futures::StreamExt;
use futures_executor::block_on;
use futures_timer::Delay;
use log::info;

#[test]
fn test_futures_thread_pool() {
    let pool = ThreadPool::new().expect("Failed to build pool");
    let (tx, rx) = mpsc::unbounded::<i32>();

    let fut_values = async {
        pool.spawn_ok(async move {
            (0..3).for_each(|v| {
                tx.unbounded_send(v).expect("Failed to send");
            })
        });
        rx.map(|v| v * 2).collect().await
    };
    let values: Vec<i32> = executor::block_on(fut_values);
    // info!("Values={values:?}");
    assert_eq!(values, [0, 2, 4].to_vec());
}

#[test]
fn test_futures_block_on() {
    use futures::executor::block_on;

    async fn hello_world() -> &'static str {
        info!("in hello world");
        "hello, world!"
    }

    let future = hello_world(); // 返回一个Future, 因此不会打印任何输出； 类型 impl Future<Output=&str>
    let output = block_on(future); // 执行`Future`并等待其运行完成，此时"hello, world!"会被打印输出
    assert_eq!(output, "hello, world!");
}

type Song = u64;

// futures 中怎么 sleep ?

async fn learn_song() -> Song {
    let song = rand::random::<u64>() % 10;
    info!("learn new song, payload: {song}");
    Delay::new(Duration::from_secs(song.clone())).await;

    info!("learnt, payload: {song}");
    song
}

async fn sing_song(song: Song) {
    info!("sing ... (payload: {song})");

    Delay::new(Duration::from_secs((&song / 2) as u64));
    info!("singing done");
}

async fn learn_and_sing() {
    // 这里使用`.await`来等待学歌的完成，但是并不会阻塞当前线程，该线程在学歌的任务`.await`后，完全可以去执行跳舞的任务
    let song = learn_song().await;

    // 唱歌必须要在学歌之后
    sing_song(song).await;
}

async fn dance() {
    let payload = rand::random::<u32>() % 17 + 3;
    info!("into dance, payload: {payload}");
    Delay::new(Duration::from_secs(payload as u64)).await;
    info!("dancing done");
}

async fn async_main() {
    let f1 = learn_and_sing();
    let f2 = dance();

    // `join!`可以并发的处理和等待多个`Future`，
    // 若`learn_and_sing Future`被阻塞，那`dance Future`可以拿过线程的所有权继续执行。
    // 若`dance`也变成阻塞状态，那`learn_and_sing`又可以再次拿回线程所有权，继续执行。
    // 若两个都被阻塞，那么`async main`会变成阻塞状态，然后让出线程所有权，并将其交给`main`函数中的`block_on`执行器
    futures::join!(f1, f2);
}

fn main() {
    std::env::set_var("RUST_LOG", "DEBUG");
    env_logger::init();

    block_on(async_main());
}