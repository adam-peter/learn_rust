fn main() {
    let first = String::from("Ferris");
    let full = add_suffix(first);
    println!("{full}");
    // println!("{full}, {first}"); - this errors - accessing moved value

    let mut s1 = String::from(":D");
    println!("{s1}");
    let s2 = s1;
    println!("{s2}");
    s1 = s2;
    println!("{s1}");

    let m1 = String::from("Hello");
    let m2 = String::from("world");
    greet(&m1, &m2);
    println!("{m1}, {m2}");

    let mut x: Box<i32> = Box::new(1); // x -> 1
    let a: i32 = *x; // 1
    *x += 1; // x -> 2
    println!("{x}, {a}"); // 2, 1

    // r1 is a reference - pointing to stack
    let r1: &Box<i32> = &x; // reference to x
    let b: i32 = **r1;
    println!("{r1}, {b}"); // 2, 2

    // r2 is a direct (immutable) reference to the value - pointing to the heap
    let r2: &i32 = &*x; // reference to the value 2 (value of x)
    let c: i32 = *r2;
    println!("{r2}, {c}"); // 2, 2

    // rust implicitly adds references / dereferences in most cases - we must add them explicitly only when calling methods on types, for example
    let x: Box<i32> = Box::new(-1);
    let x_abs1 = x.abs();
    let x_abs2 = i32::abs(*x);
    assert_eq!(x_abs1, x_abs2);

    let y: &Box<i32> = &x;
    let y_abs1 = y.abs();
    let y_abs2 = i32::abs(**y);
    assert_eq!(y_abs1, y_abs2);

    let z = String::from("Hello");
    let z_len1 = z.len();
    let z_len2 = String::len(&z);
    assert_eq!(z_len1, z_len2);

    let mut v = vec![1, 2, 3];
    let num = &mut v[2];
    *num += 1;
    println!("{num}");
    println!("{:?}", v); // 1, 2, 4

    let mut x = 1;
    let y = &x;
    let z = *y; // 1
    x += z;
    println!("{x}");
}

fn add_suffix(mut name: String) -> String {
    name.push_str(" Jr.");
    name
}

fn greet(g1: &String, g2: &String) {
    println!("{g1} {g2}!");
}