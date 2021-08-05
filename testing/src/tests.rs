use super::*;
#[test]
fn exploration() {
    assert_eq!(4, add_two(2));
}

#[test]
fn large_can_hold_small() {
    let large_one = Rectangle {
        width: 55,
        height: 89,
    };
    let small_one = Rectangle {
        width: 45,
        height: 69,
    };
    assert!(large_one.can_hold(&small_one));
}

#[test]
fn small_can_not_hold_large() {
    let large_one = Rectangle {
        width: 55,
        height: 89,
    };
    let small_one = Rectangle {
        width: 45,
        height: 69,
    };
    assert!(!small_one.can_hold(&large_one));
}

#[test]
fn greeting_with_name() {
    let name = "Carol";
    let result = greeting(name);
    assert!(result.contains("Carol"), "The greeting_name value is {}", result);
}

#[test]
#[should_panic(expected = "The value must smaller than 100")]
fn guess_than_100() {
    Guess::new(200);
}

#[test]
fn it_works() -> Result<(), String> {
    if 2 + 2 == 4 {
        Ok(())
    } else {
        Err(String::from("two plus two does not equal four!"))
    }
}