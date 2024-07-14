use std::marker::PhantomPinned;

#[derive(Debug)]
struct MyStruct {
    a: String,
    b: *const String,
    _marker: PhantomPinned, // this is !Unpin
}

fn main() {
    let mut my_struct = MyStruct {
        a: String::from("Hello"),
        b: std::ptr::null(), // initialized with null
        _marker: PhantomPinned,
    };
    my_struct.b = &my_struct.a;
    println!(
        "&my_struct.a: {:p}, my_struct: {:?}",
        &my_struct.a, my_struct
    );
    assert_eq!(&my_struct.a as *const String, my_struct.b);
    assert_eq!(my_struct.a.as_str(), unsafe { &*my_struct.b });
    // let p = &my_struct.a as *const String;

    let my1 = my_struct; // moved

    // won't compile because it's moved, borrow after being moved
    // println!("{:?}", my_struct);

    println!("moved my1: {:?}, &my1.a: {:p}", my1, &my1.a);
}
