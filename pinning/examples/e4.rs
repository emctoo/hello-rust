#![allow(unused)]
use std::marker::PhantomPinned;
use std::pin::Pin; // this helps us to make our data `!Unpin`

#[derive(Debug)]
struct SR {
    s: String,
    p: *const String,
    _marker: PhantomPinned, // we need this to make our type `!Unpin`
}

impl SR {
    fn new(txt: &str) -> Self {
        SR {
            s: String::from(txt),
            p: std::ptr::null(),
            _marker: PhantomPinned, // This makes our type `!Unpin`
        }
    }

    fn init(&mut self) {
        self.p = &self.s;
    }

    fn display(&self, name: &str) {
        println!(
            "{}: <SR s: `{}`, p: {:p} => `{}`>",
            name,
            self.s,
            &self.p,
            unsafe { &*self.p }
        );
    }
}

fn main() {
    let mut sr1 = SR::new("sr1");
    sr1.init();
    sr1.display("sr1");

    let mut sr2 = SR::new("sr2");
    sr2.init();
    sr2.display("sr2");

    let mut sr1_pin = unsafe { Pin::new_unchecked(&mut sr1) };
    let mut sr2_pin = unsafe { Pin::new_unchecked(&mut sr2) };

    {
        // E0277
        // within `SR`, the trait `Unpin` is not implemented for `PhantomPinned`
        // let x = sr1_pin.get_mut();
    }

    {
        println!("\n- before swapping");
        sr1_pin.display("sr1_pin");
        sr2_pin.display("sr2_pin");
    }

    {
        // Ok
        let _x_umr = &*sr1_pin;

        // E0596
        // cannot borrow as mutable
        // help: trait `DerefMut` is required to modify through a dereference, but it is not implemented for `Pin<&mut SR>`
        // let x = &mut *sr1_pin;

        // let y = &mut *sr2_pin;
        // std::mem::swap(x, y);
    }

    {
        let x = &mut sr1_pin;
        let y = &mut sr2_pin;
        std::mem::swap(x, y);

        // std::mem::swap(&mut sr1_pin, &mut sr2_pin);

        println!("\n- after swapping");
        sr1_pin.display("sr1_pin");
        sr2_pin.display("sr2_pin");
    }
}
