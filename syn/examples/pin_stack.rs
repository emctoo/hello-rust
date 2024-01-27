use std::pin::Pin;
use std::marker::PhantomPinned;

#[derive(Debug)]
struct Test {
    a: String,
    b: *const String,
    _marker: PhantomPinned,
}

impl Test {
    fn new(txt: &str) -> Self {
        Test {
            a: String::from(txt),
            b: std::ptr::null(),
            _marker: PhantomPinned, // This makes our type `!Unpin`
        }
    }

    fn init(self: Pin<&mut Self>) {
        let self_ptr: *const String = &self.a;
        let this = unsafe { self.get_unchecked_mut() };
        this.b = self_ptr;
    }

    fn a(self: Pin<&Self>) -> &str {
        &self.get_ref().a
    }

    fn b(self: Pin<&Self>) -> &String {
        unsafe { &*(self.b) }
    }
}

pub fn main() {
    let mut t1 = Test::new("test1");
    let mut t1_pinned = unsafe { Pin::new_unchecked(&mut t1) }; // create pinned reference
    Test::init(t1_pinned.as_mut()); // 如果不这样初始化，怎样呢？

    let mut t2 = Test::new("test2");
    let mut t2_pinned = unsafe { Pin::new_unchecked(&mut t2) };
    Test::init(t2_pinned.as_mut());

    println!("a: {}, b: {}", Test::a(t1_pinned.as_ref()), Test::b(t1_pinned.as_ref()));

    // std::mem::swap(t1_pinned.get_mut(), t2_pinned.get_mut());
    println!("a: {}, b: {}", Test::a(t2_pinned.as_ref()), Test::b(t2_pinned.as_ref()));

    // let _tmp = *t1_pinned;
    // let _tmp = t1_pinned.get_mut();
    let _tmp = t1_pinned.get_mut();
    let _tmp: &mut Test = &mut (*t1_pinned);
    // println!("a: {}, b: {}", Test::a(_tmp), Test::b(t1_pinned.as_ref()));
}