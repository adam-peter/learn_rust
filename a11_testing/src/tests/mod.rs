use crate::*;

#[test]
fn exploration() {
    assert_eq!(2 + 2, 4);
}

#[ignore]
#[test]
fn another() {
    panic!("Make this test fail");
}

#[test]
fn larget_can_hold_smaller() {
    let larger = Rectangle {
        width: 8,
        height: 7,
    };
    let smaller = Rectangle {
        width: 5,
        height: 1,
    };

    assert!(larger.can_hold(&smaller));
}

#[test]
fn smaller_cannot_hold_larger() {
    let larger = Rectangle {
        width: 8,
        height: 7,
    };
    let smaller = Rectangle {
        width: 5,
        height: 1,
    };

    assert!(!smaller.can_hold(&larger));
}