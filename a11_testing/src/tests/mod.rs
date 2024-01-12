use crate::*;

#[test]
fn it_works() {
    let result = 1 + 1;
    assert_eq!(result, 2);
}

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

#[test]
fn it_adds_two() {
    assert_eq!(add_two(2), 4);
}

#[test]
fn greeting_contains_name() {
    let result = greeting("Adam");
    // passing arguments after assertion passes them into format!() macro
    assert!(result.contains("Adam"),
            "Greeting didn't contain name, value was \"{}\"",
            result);
}

#[test]
#[should_panic(expected = "less than or equal to 100")]
fn greater_than_100() {
    // Guess::new(-1); - panics for a different reason, than we expect in the expected parameter
    Guess::new(200);
}

// tests can return Results and error on returning Error
#[test]
fn it_works_with_result() -> Result<(), String> {
    // when returning result, we can use the "?" operator inside the test fn
    if 2 + 2 == 4 {
        Ok(())
    } else {
        Err(String::from("two plus two didn't equal four"))
    }
    // we cannot use the #[should_panic] annotation, instead, use assert!(value.is_err())
}