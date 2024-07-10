#![allow(unused)]

use chrono::Local;
use std::io;
use tokio::select;
use tokio::sync::{oneshot, watch};
use tokio::time::{sleep, timeout, Duration};
use tracing::Level;
use tracing::{debug, error, info, trace, warn};
use tracing_subscriber::fmt::{format::Writer, time::FormatTime};

// select!
// - 其中一个完成的时候即完成，另外一个是被 drop (TODO: drop 实现来确认下)
// - cancellation
//

// 用来格式化日志的输出时间格式
struct LocalTimer;

impl FormatTime for LocalTimer {
    fn format_time(&self, w: &mut Writer<'_>) -> std::fmt::Result {
        write!(w, "{}", Local::now().format("%FT%T%.3f"))
    }
}

#[tokio::main]
async fn main() {
    // 直接初始化，采用默认的Subscriber，默认只输出INFO、WARN、ERROR级别的日志
    // tracing_subscriber::fmt::init();

    // 使用tracing_appender，指定日志的输出目标位置
    // 参考: https://docs.rs/tracing-appender/0.2.0/tracing_appender/
    // 如果不是在main函数中，guard必须返回到main()函数中，否则不输出任何信息到日志文件
    let file_appender = tracing_appender::rolling::daily("/tmp", "tracing.log");
    let (non_blocking, _worker_guard) = tracing_appender::non_blocking(file_appender);

    // 设置日志输出时的格式，例如，是否包含日志级别、是否包含日志来源位置、设置日志的时间格式
    // 参考: https://docs.rs/tracing-subscriber/0.3.3/tracing_subscriber/fmt/struct.SubscriberBuilder.html#method.with_timer
    let format = tracing_subscriber::fmt::format()
        .with_level(true)
        .with_target(true)
        .with_timer(LocalTimer);

    // 初始化并设置日志格式(定制和筛选日志)
    tracing_subscriber::fmt()
        .with_max_level(Level::TRACE)
        .with_writer(io::stdout) // 写入标准输出
        .with_writer(non_blocking) // 写入文件，将覆盖上面的标准输出
        .with_ansi(false) // 如果日志是写入文件，应将ansi的颜色输出功能关掉
        .event_format(format)
        .init(); // 初始化并将SubScriber设置为全局SubScriber

    trace!("it's trace ...");
    debug!("it's fun ...");
    info!("and easy ...");
    warn!("and useful ...");
    error!("but it's not enough ...");

    foo();

    // abort().await;
    // oneshot_select().await;
    // timeout_cancel().await;
    // select_timeout_cancel().await;
    watch_and_cancel().await;

    trace!("bye.\n\n");
}

fn foo() {
    info!("Hello, world!");
}

async fn abort() {
    let handle = tokio::spawn(async {
        for i in 0..10 {
            info!("tick {}", i);
            sleep(Duration::from_millis(200)).await;
        }
    });

    sleep(Duration::from_millis(500)).await;
    handle.abort();
    warn!("aborted");
}

#[derive(Debug)]
struct Notifier(String, String);

async fn watch_and_cancel() {
    let (tx, mut rx) = watch::channel::<Option<Notifier>>(None);
    let rxx = tx.subscribe();

    let handle = tokio::spawn(async move {
        let mut counter = 0;
        loop {
            select! {
                _ = rx.changed() => {
                    info!("rx changed: {:?}, exit", *rx.borrow());
                    break;
                },
                _ = sleep(Duration::from_millis(200)) => {
                    info!("tick: {}", counter);
                    counter += 1;
                }
            }
        }
        info!("task cancelled");
    });

    sleep(Duration::from_millis(1500)).await;

    info!("cancelling ...");
    let updated_value = Notifier("false".to_string(), "bad".to_string());
    tx.send(Some(updated_value)).unwrap();
    info!("cannelling signal sent");

    sleep(Duration::from_secs(2)).await;
    info!("done");
}

async fn timeout_cancel() {
    let handle = tokio::spawn(async {
        for i in 0..10 {
            info!("tick {}", i);
            sleep(Duration::from_millis(200)).await;
        }
    });

    // 用 tokio 提供的 tokio::time::timeout
    match timeout(Duration::from_millis(500), handle).await {
        Ok(_) => info!("completed"),
        Err(_) => warn!("timeout"),
    }
}

async fn select_timeout_cancel() {
    let handle = tokio::spawn(async {
        for i in 0..10 {
            info!("tick {}", i);
            sleep(Duration::from_millis(200)).await;
        }
    });

    select! {
        _ = handle => info!("completed"),
        _ = sleep(Duration::from_millis(300)) => warn!("timeout"),
    }
}

async fn oneshot_select() {
    let (tx, rx) = oneshot::channel::<u64>();

    let t1 = tokio::spawn(async move {
        let sec = rand::random::<u64>() % 5;
        info!("sleeping for {} seconds", sec);
        sleep(Duration::from_secs(sec)).await;

        let _ = tx.send(1);
        info!("sent");

        // sleep(Duration::from_secs(1)).await;
        info!("task 1 ends");
    });

    let t2 = tokio::spawn(async move {
        match rx.await {
            Ok(v) => {
                info!("received: {}", v);
                sleep(Duration::from_secs(v.into())).await;

                info!("task 2 ends");
            }
            Err(e) => error!("error: {:?}", e),
        }
    });

    select! {
        _ = t1 => info!("task 1 completed"),
        _ = t2 => info!("task 2 completed"),
    }

    info!("done");
}
