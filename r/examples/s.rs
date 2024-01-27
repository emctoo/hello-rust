use std::{io, thread, time};
use std::collections::HashMap;
use std::io::BufReader;
use std::io::prelude::*;
use std::net::{Shutdown, TcpListener, TcpStream};
use std::ops::Index;
use std::sync::mpsc;

use log::{debug, error, info};
use uuid;

// https://redis.io/docs/reference/protocol-spec/
// Simple strings 	RESP2 	Simple 	    +
// Simple Errors 	RESP2 	Simple 	    -
// Integers 	    RESP2 	Simple 	    :
// Bulk strings 	RESP2 	Aggregate 	$
// Arrays 	        RESP2 	Aggregate 	*

enum Command {
    Create { id: uuid::Uuid, host: String, port: u16, stream: TcpStream},
    Quit { id: uuid::Uuid }
}

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() >= 2 && args.index(1) == "client" {
        client();
        return;
    }

    std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    let mut clients = HashMap::<uuid::Uuid, TcpStream>::new();

    let (tx, rx) = mpsc::channel::<Command>();

    let tx_pipe = tx.clone();
    thread::spawn(move || { listen("127.0.0.1", 2024, tx_pipe); });
    info!("listening at 127.0.0.1:2024 ...");

    loop {
        match rx.recv() {
            Ok(cmd) => {
                match cmd {
                    Command::Create {id, host: client_host, port: client_port, stream} => {
                        clients.insert(id, stream);
                        info!("client added, from {}:{}, id: {}", client_host, client_port, id);
                        // connections += 1
                    }
                    Command::Quit { id } => {
                        clients.remove(&id);
                        info!("client removed, id: {}, total: {}", id, clients.len());
                        // break
                    }
                }
            }
            Err(e) => {
                info!("Pipe broken - {}", e);
            }
        }
    }

    // bye
}

fn listen(host: &str, port: u16, tx: mpsc::Sender<Command>){
    let addr = format!("{}:{}", host, port);
    let listener = TcpListener::bind(addr).unwrap(); // iterator, which encapsulates `accept`
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let client_addr = stream.peer_addr().unwrap();
                let client_host = client_addr.ip();
                let client_port = client_addr.port();
                let id = uuid::Uuid::new_v4();

                info!("new connection: {}:{}, id: {}", client_host, client_port, id);

                let txp = tx.clone();
                let stream_clone = stream.try_clone().unwrap();
                thread::spawn(move || { connect_handler(id, stream, txp); });
                tx.send(Command::Create { id, host: client_host.to_string(), port: client_port, stream: stream_clone }).unwrap();
            }
            Err(e) => error!("error: {}", e)
        }
    }

    drop(listener);
}

fn connect_handler(id: uuid::Uuid, mut stream: TcpStream, tx: mpsc::Sender<Command>) {
    let mut buf = BufReader::new(stream.try_clone().unwrap());
    loop {
        let mut s = String::new();

        match buf.read_line(&mut s){
            Ok(sz)  => {
                if sz == 0 { break; } // EOF
                debug!("received data, id: {}, {} bytes, {}", id, sz, s);

                if s == "ping\n" {
                    stream.write("pong".as_bytes()).unwrap();
                    debug!("{}, receive ping, pong sent", id);
                }

                // if s.contains("quit") {
                if s == "quit\n" || s == "bye\n" {
                    tx.send(Command::Quit { id }).unwrap();
                    break;
                }
            }
            Err(e) => info!("Error receiving data! - {}", e)
        }
    }

    info!("Client {}:{} dropped", stream.peer_addr().unwrap().ip(), stream.peer_addr().unwrap().port());
    tx.send(Command::Quit { id }).unwrap();

    stream.shutdown(Shutdown::Both).unwrap();
}

fn client() {
    // print!("log: {}", std::env::var("RUST_LOG").unwrap());

    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let host = "127.0.0.1";
    let port = 2024;

    let mut stream = TcpStream::connect((host, port)).unwrap();
    let mut input_stream = stream.try_clone().unwrap();

    thread::spawn(move || {
        let mut client_buffer = [0u8; 1024];

        const PONG: &[u8] = b"pong\n";
        loop {
            match input_stream.read(&mut client_buffer) {
                Ok(0) => std::process::exit(0),
                Ok(n) => {
                    // match 这个应该效率更高，但是有点 tricky
                    // match client_buffer.as_slice() {
                    //     PONG => {
                    //         debug!("pong received, len: {}", n);
                    //         continue;
                    //     },
                    //     _ => {
                    //         io::stdout().write(&client_buffer).unwrap();
                    //         io::stdout().flush().unwrap();
                    //     }
                    // }

                    let result = std::str::from_utf8(&client_buffer);
                    if result.is_err() {
                        error!("fail to decode bytes: {:?}", result.err());
                        continue
                    }
                    let content = result.unwrap().trim();
                    info!("content: [{}]", content);
                    if client_buffer.starts_with("pong".as_bytes()) {
                        debug!("pong received, len: {}", n);
                        continue;
                    }
                    io::stdout().write(&client_buffer).unwrap();
                    io::stdout().flush().unwrap();
                },
                Err(error) => panic!("error: {}", error)
            }
        }
    });

    let mut output_stream_heartbeat = stream.try_clone().unwrap();
    thread::spawn(move || {
        info!("launch heartbeat thread ...");
        loop {
            let payload = "ping\n".as_bytes();
            output_stream_heartbeat.write(payload).unwrap();
            output_stream_heartbeat.flush().unwrap();
            thread::sleep(time::Duration::from_secs(1));
            debug!("heartbeat sent");
        }
    });

    let output_stream = &mut stream;
    let mut user_buffer = String::new();
    loop {
        io::stdin().read_line(&mut user_buffer).unwrap();

        output_stream.write(user_buffer.as_bytes()).unwrap();
        output_stream.flush().unwrap();
    }
}
