#![feature(negative_impls)]

use std::pin::Pin;

// use std::marker::PhantomPinned;
use thiserror::Error;

#[derive(Clone, Debug, Error)]
#[error("value error")]
struct PS {
    s: String, // heap
               // _marker: PhantomPinned,
}

fn main() {
    sp();
}
fn sp() {
    let ps0 = PS {
        s: "hello, world!".to_owned(),
        // _marker: PhantomPinned,
    };
    println!("{:?}, {:p} {:p}", ps0, &ps0, &ps0.s);

    // let ps1 = Box::pin(&ps0);
    let ps1 = Pin::new(&ps0);
    println!("{:?}, {:p} {:p}", ps1, &ps1, &ps1.s);

    let ps2 = ps1; // moved
    println!("{:?}, {:p} {:p}", ps2, &ps2, &ps2.s);

    let ps3 = ps2; // moved
    println!("{:?}, {:p} {:p}", ps3, &ps3, &ps3.s);
}
