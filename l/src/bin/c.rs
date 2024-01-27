use std::io::{Cursor, Write};

fn main() {
    let mut buf = Cursor::new(vec![0; 16]);
    let n = buf.write(&vec![32u8; 1]);
    println!("n: {}, buf: {:?}", n.unwrap(), buf.get_ref());
}
