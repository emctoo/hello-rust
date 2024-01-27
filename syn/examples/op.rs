use std::num::{IntErrorKind, ParseIntError};

fn try_to_parse() -> Result<i32, ParseIntError> {
    let x: i32 = "123".parse()?; // x = 123
    let y: i32 = "24a".parse()?; // returns an Err() immediately
    Ok(x + y)                    // Doesn't run.
}

#[test]
fn test_try_to_parse() {
    let res = try_to_parse();
    assert!(res.is_err());
    assert_eq!(*res.err().unwrap().kind(), IntErrorKind::InvalidDigit);
}

#[test]
fn test_matches_macro() {
    assert!(matches!("23".parse::<u8>(), Ok(_)));
    assert!(!matches!("23".parse::<u8>(), Ok(24)));
    assert!(matches!("a23".parse::<u8>(), Err(_)));
}

//
// fn f((Ok(i) | Err(i)): Result<i32, i32>) -> i32 { i }
//
// #[test]
// fn test_f() {
//     assert_eq!(f(Ok(1)), 1);
// }

#[test]
fn test_binary_search() {
    let (Ok(i) | Err(i)) = [1, 2, 3].binary_search(&2);
}

fn main() {
    if matches!("23".parse::<usize>(), Ok(_)) {
        println!("It's a number!");
    }
}
