use std::fs::OpenOptions;
use tracing::{debug, error, info, Level, Metadata};
use tracing_subscriber::{
    filter::LevelFilter,
    fmt,
    layer::{Context, Filter, Layer},
    prelude::*,
    Registry,
};

struct DebugOnlyFilter;
impl<S> Filter<S> for DebugOnlyFilter {
    fn enabled(&self, meta: &Metadata<'_>, _: &Context<'_, S>) -> bool {
        meta.level() == &Level::DEBUG
    }
}

#[tokio::main]
async fn main() {
    let error_file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("/tmp/tracing-error.log")
        .unwrap();
    let debug_file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("/tmp/tracing-debug.log")
        .unwrap();

    let subscriber = Registry::default()
        // .with(
        //     fmt::layer().compact().with_ansi(true), // stdout layer, to view everything in the console
        // )
        .with(
            fmt::layer()
                .json()
                .with_writer(error_file)
                .with_filter(LevelFilter::from_level(Level::ERROR)),
        )
        .with(
            fmt::layer()
                .json()
                .with_writer(debug_file)
                .with_filter(DebugOnlyFilter),
            // .with_filter(filter::LevelFilter::from_level(Level::DEBUG)), // any greater than or equal to DEBUG
        );

    tracing::subscriber::set_global_default(subscriber).unwrap();

    debug!("restarting ...");

    error!("test");
    info!("test2");
    debug!("test3");
}
