#![allow(unused)]

use std::fmt::{Display, Formatter};
use std::marker::PhantomPinned;
use std::pin::Pin;

#[derive(Debug)]
struct SR {
    s: String,
    p: *const String,
    _marker: PhantomPinned,
}

impl SR {
    fn new(txt: &str) -> Pin<Box<Self>> {
        let sr = SR {
            s: String::from(txt),
            p: std::ptr::null(),
            _marker: PhantomPinned,
        };

        let mut boxed = Box::pin(sr);
        let self_ptr: *const String = &boxed.as_ref().s;
        unsafe { boxed.as_mut().get_unchecked_mut().p = self_ptr };

        boxed
    }

    fn s<'a>(self: Pin<&'a Self>) -> &'a str {
        &self.get_ref().s
    }

    fn p<'a>(self: Pin<&'a Self>) -> &'a String {
        unsafe { &*(self.p) }
    }
}

impl Display for SR {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "<SR s: `{}`, p: {:p} => {}", self.s, self.p, unsafe {
            &*self.p
        })
    }
}

pub fn main() {
    let mut sr1 = SR::new("sr1");
    let mut sr2 = SR::new("sr2");

    println!("sr1, s: {}, p: {}", sr1.as_ref().s(), sr1.as_ref().p());
    println!("sr2, s: {}, p: {}", sr2.as_ref().s(), sr2.as_ref().p());

    // let sr1_mr = sr1.as_mut();
    // let sr2_mr = sr2.as_mut();
    // std::mem::swap(sr1_mr, sr2_mr);

    // std::mem::swap(sr1.get_mut(), sr2.get_mut());

    {
        println!("\n- original");
        println!("sr1: {}", sr1);
        println!("sr2: {}", sr2);
        std::mem::swap(&mut sr1, &mut sr2);

        println!("\n- swapped");
        println!("sr1: {}", sr1);
        println!("sr2: {}", sr2);
    }

    // println!("sr1, s: {}, p: {}", sr1.as_ref().s(), sr1.as_ref().p());
    // println!("sr2, s: {}, p: {}", sr2.as_ref().s(), sr2.as_ref().p());

    // std::mem::swap(&mut *sr1, &mut *sr2);
}
