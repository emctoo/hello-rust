use std::pin::Pin;
fn main() {}

#[test]
fn test_pinned_moving() {
    let mut x: u8 = 42;
    let pin_x = Pin::new(&mut x);
    let y = pin_x; // move 
    assert!(true);
}


struct Foo {
    x: u8,
}

// #[test]
// fn test_pin_future() {
//     let mut future = async { 1 };
//     let pin_future = Pin::new(&mut future);
//
//     // pin_future 不能安全地移动，因为它没有实现 Unpin
//     let y = pin_future;  // 编译错误
//     assert!(true);
// }
