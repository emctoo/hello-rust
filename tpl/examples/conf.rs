use config::Config;
use std::collections::HashMap;

// TODO add deserialization
#[derive(Debug)]
struct Conf {
    host: String,
    debug: bool,
    port: u16,
}

fn main() {
    let settings = Config::builder()
        .add_source(config::File::with_name("examples/conf.toml"))
        .add_source(config::Environment::with_prefix("APP"))
        .build()
        .unwrap();

    println!(
        "{:?}",
        settings
            .try_deserialize::<HashMap<String, String>>()
            // .try_deserialize::<Conf>()
            .unwrap()
    );
}
