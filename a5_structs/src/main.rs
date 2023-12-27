#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("badmood111"),
        email: String::from("my@email.com"),
        sign_in_count: 1,
    };
    println!("{user1:?}");
    println!("{}", user1.email);
    user1.active = false;
    println!("{}", user1.active);

    let user2 = build_user(String::from("test@email"), String::from("test"));
    println!("{user2:?}");

    // struct update syntax
    /*let user3 = User {
        username: String::from("testtest"),
        active: user2.active,
        sign_in_count: user2.sign_in_count,
        email: user2.email
    };*/
    let user3 = User {
        username: String::from("testtest"),
        ..user2
    };
    println!("{user3:?}");

    // tuple structs
    #[derive(Debug)]
    struct Point(i32, i32, i32);
    let mut p1 = Point(0, 0, 0);
    let x = &mut p1.0;
    *x += 1;
    println!("{p1:?}");

    // unit-like structs - for when you don't want any data, but want to implement a trait on a type
    struct Cat;
    impl Cat {
        fn meow(self) {
            println!("meow :3");
        }
    }
    let subject = Cat;
    subject.meow();
}

fn build_user(email: String, username: String) -> User {
    // struct init shorthand
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}