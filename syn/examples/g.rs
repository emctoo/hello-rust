#![allow(unused)]
#![feature(auto_traits, negative_impls, generators, generator_trait, never_type)]


use std::ops::{Generator, GeneratorState};
use std::pin::Pin;

fn main() {
    // test_g();
    // test_g1();
    test_g2();
}

fn test_g1() {
    let mut g = || {
        yield "hi";
        ()
    };

    while let GeneratorState::Yielded(value) = Pin::new(&mut g).resume(()) {
        println!("value: {value}");
    }
}

fn test_g2() {
    fn map<T, U>(mut f: impl FnMut(T) -> U) -> impl Generator<T, Yield=U, Return=!> + Unpin {
        move |mut t| loop {
            t = yield f(t);
        }
    }
    
    let mut gen = Box::pin(map(|x| x * 2));
    dbg!(gen.as_mut().resume(1));
    dbg!(gen.as_mut().resume(2));
    dbg!(gen.as_mut().resume(3));
}

fn test_gen_impl() {
    enum GeneratorA {
        Enter,
        Counter { state: u8 },
        Exit,
    }

    impl GeneratorA {
        fn new() -> Self { GeneratorA::Enter } // created in `Enter` state
    }

    impl ! Unpin for GeneratorA {}

    impl Generator<u32> for GeneratorA {
        type Yield = u8;
        type Return = ();

        fn resume(mut self: Pin<&mut Self>, limit: u32) -> GeneratorState<Self::Yield, Self::Return> {
            let this = unsafe { self.get_unchecked_mut() };
            match *this {
                GeneratorA::Enter => {
                    *this = GeneratorA::Counter { state: 0 };
                    GeneratorState::Yielded(0)
                }
                GeneratorA::Counter { state } => {
                    if state == limit as u8 {
                        GeneratorState::Complete(())
                    } else {
                        *this = GeneratorA::Counter { state: state + 1 };
                        GeneratorState::Yielded(state + 1)
                    }
                }
                GeneratorA::Exit => {
                    panic!("already exited")
                }
            }
        }
    }

    let mut g1 = Box::pin(GeneratorA::new());
    println!("yield {:?}", g1.as_mut().resume(3));
    println!("yield {:?}", g1.as_mut().resume(3));
    println!("yield {:?}", g1.as_mut().resume(3));
    println!("yield {:?}", g1.as_mut().resume(3));
    println!("yield {:?}", g1.as_mut().resume(3));
    println!("yield {:?}", g1.as_mut().resume(3));

    let mut g2 = Box::pin(GeneratorA::new());
    while let GeneratorState::Yielded(num) = g2.as_mut().resume(7) {
        println!("num: {num}");
    }
}