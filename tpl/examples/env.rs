#![allow(unused)]
use dotenvy::Error;
// use std::error::Error;
// use std::io::Result;

fn main() -> Result<(), dotenvy::Error> {
    dotenvy::dotenv()?;

    let mut max_key_sz = 0;
    for (key, value) in std::env::vars() {
        max_key_sz = if key.len() >= max_key_sz {
            key.len()
        } else {
            max_key_sz
        };
        // println!("{:03} {:28} => {}", key.len(), key, value);
    }

    // println!("final sz: {}", max_key_sz);
    println!("FOO => {}", std::env::var("FOO").unwrap());
    println!("BAR => {}", std::env::var("BAR").unwrap());

    Ok(())
}
