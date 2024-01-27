#![allow(unused)]

fn main() {}

#[test]
fn test_return_impl_trait() {
    trait Animal {
        fn name(&self) -> String { "animal".to_owned() }
    }

    struct Sheep {}
    impl Animal for Sheep {
        fn name(&self) -> String { "sheep".to_owned() }
    }

    struct Dog {}
    impl Animal for Dog {
        fn name(&self) -> String { "dog".to_owned() }
    }

    // THIS WON'T WORK
    // fn running_animal() -> impl Run {
    //     let random = true;
    //     if random {
    //         Dog {}
    //     } else {
    //         Sheep {} // compile-time error, as `impl Trait` is static
    //     }
    // }

    fn dynamic_animal(random: bool) -> Box<dyn Animal> {
        if random {
            Box::new(Dog {})
        } else {
            Box::new(Sheep {})
        }
    }

    assert_eq!(&dynamic_animal(true).as_ref().name(), "dog");
    assert_eq!(&dynamic_animal(false).as_ref().name(), "sheep");
}