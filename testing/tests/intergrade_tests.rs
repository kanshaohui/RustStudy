use testing;

mod common;
#[test]
fn small_can_not_hold_large() {
    common::setup();
    let large_one = testing::Rectangle{
        width: 55,
        height: 89,
    };
    let small_one = testing::Rectangle{
        width: 45,
        height: 69,
    };
    assert!(!small_one.can_hold(&large_one));
}
