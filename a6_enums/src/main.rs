#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

#[derive(Debug)]
enum IpAddrKind2 {
    V4(String),
    V6(String),
}

#[derive(Debug)]
enum IpAddrKind3 {
    V4(u8, u8, u8, u8),
    V6(String),
}

#[derive(Debug)]
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

enum Message {
    Quit, // struct Quit;
    Move { x: i32, y: i32 }, // struct Move {x: i32, y: i32}
    // Move (MoveStruct), - same
    Write(String), // struct Write(String);
    ChangeColor(i32, i32, i32), // struct ChangeColor(i32, i32, i32);
}

impl Message {
    fn call(&self) {
        () // ...
    }
}

fn main() {
    let ip_kind4 = IpAddrKind::V4;
    let ip_kind6 = IpAddrKind::V6;
    route(ip_kind4);
    route(ip_kind6);

    let ip4 = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    let ip6 = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
    println!("{ip4:?}, {ip6:?}");

    // data inside enums
    let home = IpAddrKind2::V4(String::from("127.0.0.1"));
    let loopback = IpAddrKind2::V6(String::from("::1"));
    println!("{home:?}, {loopback:?}");

    // different data types for enum values
    let home = IpAddrKind3::V4(127, 0, 0, 1);
    println!("{home:?}");

    // implementing associated functions / methods on enums
    let m = Message::Write(String::from("hello"));
    m.call();

    // Option stdlib enum
    let some_number = Some(5);
    let some_char = Some('c');
    let absent_number: Option<i32> = None;
    println!("{some_number:?}, {some_char:?}, {absent_number:?}");

    let num1 = 2;
    let num2 = Some(2);
    // println!("{}", num1 + num2); - errors, cannot add i32 and Option<i32>
    println!("{}", num1 + num2.unwrap());
}

fn route(ip_kind: IpAddrKind) {
    println!("Ip address {ip_kind:?}");
}