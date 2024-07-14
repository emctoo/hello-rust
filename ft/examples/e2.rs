use std::io::Error;
use std::ops::Add;
use std::pin::Pin;
use std::task::{Context, Poll};

use clap::Parser;
use futures::FutureExt;
use tokio::fs::File;
use tokio::io::{AsyncRead, AsyncReadExt, ReadBuf};
use tokio::time::{sleep, Duration, Instant, Sleep};
use tracing::info;

#[derive(Parser)]
enum Cli {
    Default,
    Slow,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    // std::env::set_var("RUST_LOG", "DEBUG");
    // env_logger::init();
    tracing_subscriber::fmt::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let mut buf = vec![0; 1024 * 1024];
    let before = Instant::now();

    match Cli::parse() {
        Cli::Default => {
            File::open("/dev/urandom")
                .await?
                .read_exact(&mut buf)
                .await?
        }
        Cli::Slow => {
            SlowRead::new(File::open("/dev/urandom").await?)
                .read_exact(&mut buf)
                .await?
        }
    };
    info!("Read {} bytes in {:?}", buf.len(), before.elapsed());
    Ok(())
}

struct SlowRead<R> {
    sleep: Pin<Box<Sleep>>,
    reader: Pin<Box<R>>,
}

impl<R> SlowRead<R> {
    fn new(reader: R) -> Self {
        SlowRead {
            sleep: Box::pin(sleep(Duration::from_secs(1))),
            reader: Box::pin(reader),
        }
    }
}

impl<R> AsyncRead for SlowRead<R>
where
    R: AsyncRead,
{
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<Result<(), Error>> {
        // sleep for 1 second
        match self.sleep.poll_unpin(cx) {
            Poll::Ready(_) => {
                info!("READY");
                self.sleep
                    .as_mut()
                    .reset(Instant::now().add(Duration::from_secs(1)));
                self.reader.as_mut().poll_read(cx, buf)
            }
            Poll::Pending => {
                info!("not ready yet");
                Poll::Pending
            }
        }
    }
}
