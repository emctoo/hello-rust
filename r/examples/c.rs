#![allow(unused_variables)]
#![allow(unused_imports)]

use std::io::{self, Read, Write};
use std::net::TcpStream;
use std::{thread, time};
use log::{debug, info};

fn main() {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    // let host = "127.0.0.1";
    // let port = 2024;
    //
    // let mut stream = TcpStream::connect((host, port)).unwrap();
    // let mut input_stream = stream.try_clone().unwrap();
    //
    // let handler = thread::spawn(move || {
    //     let mut client_buffer = [0u8; 1024];
    //
    //     loop {
    //         match input_stream.read(&mut client_buffer) {
    //             Ok(0) => std::process::exit(0),
    //             Ok(n) => {
    //                 io::stdout().write(&client_buffer).unwrap();
    //                 io::stdout().flush().unwrap();
    //             },
    //             Err(error) => panic!("{}", error)
    //         }
    //     }
    // });
    //
    // let mut output_stream_heartbeat = stream.try_clone().unwrap();
    // let heartbeat_thread = thread::spawn(move || {
    //     loop {
    //         output_stream_heartbeat.write("ping\n".as_bytes()).unwrap();
    //         output_stream_heartbeat.flush().unwrap();
    //         thread::sleep(time::Duration::from_secs(1));
    //         debug!("heartbeat sent");
    //     }
    // });
    //
    // let output_stream = &mut stream;
    // let mut user_buffer = String::new();
    // loop {
    //     io::stdin().read_line(&mut user_buffer).unwrap();
    //
    //     output_stream.write(user_buffer.as_bytes()).unwrap();
    //     output_stream.flush().unwrap();
    // }
}