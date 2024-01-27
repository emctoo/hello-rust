use std::{path::Path, sync::mpsc::channel};

use notify::EventKind;
use notify::{Event, RecommendedWatcher, Watcher};

fn main() {
    let (tx, rx) = channel();
    let mut watcher = RecommendedWatcher::new(tx, notify::Config::default()).unwrap();
    watcher
        .watch(
            Path::new("examples/conf.toml"),
            notify::RecursiveMode::NonRecursive,
        )
        .unwrap();

    println!("watching ...");
    loop {
        match rx.recv() {
            // Ok(Ok(Event { kind, paths, .. })) => println!("kind: {:?}, {:?} changes", kind, paths),
            Ok(Ok(Event {
                kind: EventKind::Modify(_),
                paths,
                ..
            })) => println!("modify {:?}", paths),
            // Ok(Ok(ev)) => println!("ev: {:?}", ev),
            // Ok(Err(err)) => println!("error: {:?}", err),
            // Err(err) => println!("error: {:?}", err),
            _ => println!("not interested"),
        }
    }
}
