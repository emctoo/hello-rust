#![allow(unused)]

use std::fmt::{Display, Formatter, Pointer, Write};
use std::marker::{PhantomPinned, PhantomData};
use std::pin::Pin;
use std::ptr;
use std::ptr::{addr_of, addr_of_mut};

fn main() {
    // t_addr();
    // t_pin();
    t_pin_on_heap();
}

fn t_pin_on_heap() {
    #[derive(Debug)]
    struct Test {
        s: String,
        p: *const String,
        _marker: PhantomPinned,
    }

    impl Test {
        fn new(s: &str) -> Pin<Box<Self>> {
            let t = Self { s: s.to_owned(), p: ptr::null(), _marker: PhantomPinned };
            let mut boxed = Box::pin(t);
            unsafe {
                let p = boxed.as_mut().get_unchecked_mut();
                p.p = &p.s;
            }
            boxed
        }
    }

    impl Display for Test {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            assert_eq!(self.p, addr_of!(self.s));
            write!(f, "Test[s({:?}): {:?}, p: {:?}]", addr_of!(self.s), &self.s, self.p)
        }
    }

    let t = Test::new("hello");
    println!("t: {}", t);
    let tt = t;
    println!("u: {}", tt);
}

fn t_addr() {
    #[derive(Debug, Default, Clone, Copy)]
    #[repr(C)]
    struct S {
        aligned: u8,
        unaligned: u32,
    }

    let s = S::default();
    let s_ptr: *const S = &s;
    println!("s: {s:?} at {s_ptr:?}");
    println!("a: {:?}, u: {:?}", ptr::addr_of!(s.aligned), ptr::addr_of!(s.unaligned));

    // mutable version
    let mut s_mut = S::default();
    println!("{s_mut:?}");
    let s_ptr_mut: *mut S = &mut s_mut;
    unsafe {
        let u_addr: *mut u32 = addr_of_mut!((*s_ptr_mut).unaligned);
        *u_addr = 9527;
    }

    let a_addr = addr_of_mut!(s_mut.aligned);
    unsafe { *a_addr = 128 as u8 }
    println!("{s_mut:?}");
}

fn t_pin() {
    #[derive(Debug)]
    struct S {
        s: String,
        p: *const String,
        _marker: PhantomPinned,
    }

    impl S {
        fn new(s: &str) -> Self {
            Self {
                s: s.to_owned(),
                p: ptr::null(),
                _marker: PhantomPinned,
            }
        }

        fn init(&mut self) {
            self.p = &self.s;
        }

        fn init_pined(self: Pin<&mut S>) {
            // self.p = unsafe { &self.get_unchecked_mut().s };
            let this = unsafe { self.get_unchecked_mut() };
            this.p = &this.s;
        }
    }

    let mut s = S::new("hello");
    s.init();
    println!("s: {:?}", s);

    let p = s; // moved
    println!("p: {p:?}, p.s addr: {:?}", addr_of!(p.s)); // 有问题的

    // new test
    let mut new_s = S::new("world");
    println!("new_s: {new_s:?}, addr_of(.s): {:?}", addr_of!(new_s.s));

    // 创建新的 pin 住的值
    let mut new_s_pined = unsafe { Pin::new_unchecked(&mut new_s) };
    println!("new_s: {new_s_pined:?}, addr_of(.s): {:?}", addr_of!(new_s_pined.s));

    S::init_pined(new_s_pined.as_mut()); // 到这里是pin住了
    println!("new_s_pined: {:?}, addr_of(.s): {:?}", new_s_pined, addr_of!(new_s_pined.s));

    let moved_from_new_s_pined = new_s_pined;
    println!("moved: {:?}, addr_of(.s): {:?}", moved_from_new_s_pined, addr_of!(moved_from_new_s_pined.s));
    assert_eq!(addr_of!(moved_from_new_s_pined.s), moved_from_new_s_pined.p);

    // println!("new_s_pined: {:?}, addr_of(.s): {:?}", new_s_pined, addr_of!(new_s_pined.s));
}

