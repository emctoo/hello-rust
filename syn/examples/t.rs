#![allow(unused)]
use std::{io::Write, result};

fn main() -> std::io::Result<()> {
    // let mut buf = [0u8; 1024 * 16];
    let mut buf = Vec::<u8>::new();
    let _ = say_hello(&mut buf);

    println!("{:?}", buf);

    let v = 1;
    let u: u8 = 42;
    demo1(v);
    demo1::<u8>(u); // turbo-fish

    demo2(v);
    // demo2 是没有泛型参数的，不同使用 turbo fish 的写法
    // demo2::<u8>(u);

    let _r1 = demo_return_box();
    let _r2 = demo_return_impl();
    // println!("result: {:?}, {:?}", result, demo_return_impl());
    Ok(())
}

fn say_hello(out: &mut dyn Write) -> std::io::Result<()> {
    let _ = out.write_all(b"hello, world!");
    out.flush()
}

trait Tr {}
impl Tr for i32 {}
impl Tr for u8 {}

fn demo1<T: Tr>(_t: T) {
    println!("demo1");
}

// 没有泛型参数
fn demo2(_t: impl Tr) {
    println!("demo2");
}

fn demo_return_box() -> Box<dyn Tr> { Box::new(1024) }
fn demo_return_impl() -> impl Tr { 99 }
