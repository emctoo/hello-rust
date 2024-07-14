use std::pin::Pin;
use tracing::info;

#[derive(Debug)]
struct Foo {
    x: i32,
    y: i32,
}

impl Foo {
    fn new() -> Self {
        Foo { x: 0, y: 1 }
    }
}

fn main() {
    tracing_subscriber::fmt::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    // 先创建数据在堆上的 Box<Foo> 指针，然后在基于 Box<Foo> 创建 Pin 指针
    let box_foo: Box<Foo> = Box::new(Foo::new());

    // struct Pin<T> { ... }
    // Pin is smart pointer, and the Box is also a smart pointer
    let pin_foo: Pin<Box<Foo>> = Pin::new(box_foo);

    // 这里必须使用 rerference，否则 borrow checker 将会报错
    // *pin_foo 相当于获取 Foo
    let foo_ref = &*pin_foo;
    info!("{:?}", foo_ref);
}

#[test]
fn test_pinned_moving() {
    let mut x: u8 = 42;
    let pin_x = Pin::new(&mut x);
    let y = pin_x;
}
