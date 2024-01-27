use std::marker::PhantomPinned;
use std::pin::Pin;

#[derive(Debug)]
struct MyStruct {
    a: String,
    b: *const String,
    _pin_marker: PhantomPinned,
}

impl MyStruct {
    fn new(content: &str) -> Pin<Box<Self>> {
        let my_struct = Self { a: String::from(content), b: std::ptr::null(), _pin_marker: PhantomPinned };
        let mut pinned_my_struct = Box::pin(my_struct); // Box::pin

        // unsafe { pinned_my_struct.as_mut().get_unchecked_mut() }.b = &pinned_my_struct.a;
        unsafe { pinned_my_struct.as_mut().get_unchecked_mut().b = &pinned_my_struct.a; }

        pinned_my_struct
    }

    fn a(self: Pin<&Self>) -> &str {
        &self.get_ref().a
    }

    fn b(self: Pin<&Self>) -> &str {
        unsafe { &*self.b }
    }
}


fn main() {
    let my1 = MyStruct::new("Hello");
    println!("my1: {:?}, &my1.a: {:p}", my1, &my1.a);

    let my2 = MyStruct::new("World");
    println!("my2: {:?}, &my2.a: {:p}", my2, &my2.a);

    // std::ptr::swap(my1.as_mut().get_mut(), my2.as_mut().get_mut());
    // std::mem::swap(my1.as_mut().get_mut(), my2.as_mut().get_mut());
    // std::mem::swap(&mut *my1, &mut *my2);
}

// self referential
// generator
// async/await programming