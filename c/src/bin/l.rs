use chrono::Local;
use log::info;
use log::*;
use std::fs::File;
use std::io::Write;

fn _simple_env_logger() {
    env_logger::init();
}

fn _env_logger_to_file() {
    let target = Box::new(File::create("/tmp/rust-c.log").expect("Can't create file"));
    env_logger::Builder::new()
        .target(env_logger::Target::Pipe(target))
        .filter(None, LevelFilter::Debug)
        .format(|buf, record| {
            writeln!(
                buf,
                "[{} {} {}:{}] {}",
                Local::now().format("%Y-%m-%d %H:%M:%S%.3f"),
                record.level(),
                record.file().unwrap_or("unknown"),
                record.line().unwrap_or(0),
                record.args()
            )
        })
        .init();
}

fn main() {
    log4rs::init_file("config/log4rs.yaml", Default::default()).unwrap();
    // let config_str = include_str!("config/log4rs.yml");
    // let config = serde_yaml::from_str(config_str).unwrap();
    // log4rs::init_raw_config(config).unwrap();

    let handle = std::thread::spawn(f);
    handle.join().unwrap();
}

fn f() {
    info!("hello from thread {:?}", std::thread::current());
    std::thread::sleep(std::time::Duration::from_secs(1));
    info!("done");
}
