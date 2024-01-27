use std::marker::PhantomPinned;
use std::pin::Pin;

#[derive(Debug)]
struct SR {
    s: String,
    p: *const String,
    _marker: PhantomPinned,
}

impl SR {
    fn new(txt: &str) -> Self {
        SR {
            s: String::from(txt),
            p: std::ptr::null(),
            _marker: PhantomPinned,
        }
    }

    fn init<'a>(self: Pin<&'a mut Self>) {
        let self_ptr: *const String = &self.s;
        let this = unsafe { self.get_unchecked_mut() };
        this.p = self_ptr;
    }

    fn s<'a>(self: Pin<&'a Self>) -> &'a str {
        &self.get_ref().s
    }

    fn p<'a>(self: Pin<&'a Self>) -> &'a String {
        unsafe { &*(self.p) }
    }
}

pub fn main() {
    let mut sr1 = SR::new("sr1");
    let sr1_pin = unsafe { Pin::new_unchecked(&mut sr1) };
    sr1_pin.init();

    // let mut sr2 = SR::new("sr2");
    // let sr2_pin = unsafe { Pin::new_unchecked(&mut sr2) };
    // sr2_pin.init();

    let sr1_pin_mr = sr1_pin.get_mut();
    // std::mem::swap(sr1_pin.get_mut(), sr2_pin.get_mut());

    // let mut sr1 = unsafe { Pin::new_unchecked(&mut SR::new("sr1")) };
    // SR::init(sr1.as_mut());
    // sr1.init();

    // let mut sr2 = unsafe { Pin::new_unchecked(&mut SR::new("sr2")) };
    // SR::init(sr2.as_mut());
    // println!("a: {}, b: {}", SR::s(sr1.as_ref()), SR::p(sr1.as_ref()));

    // std::mem::swap(test1.get_mut(), test2.get_mut());
    // println!("a: {}, b: {}", SR::s(test2.as_ref()), SR::p(test2.as_ref()));
}
