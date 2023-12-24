fn main() {
    let mut number = 3;
    loop {
        if number == 0 { break }
        println!("{number}!");
        number -= 1;
    }
    println!("LIFTOFF!!!");

    let mut number = 3;
    while number != 0 {
        println!("{number}!");
        number -= 1;
    }
    println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < 5 {
        println!("{}", a[index]);
        index += 1;
    }
    for item in a {
        println!("{item}");
    }

    for i in 1..=10 {
        println!("Counting... {i}");
    }

    for i in (1..=3).rev() {
        println!("{i}!");
    }
    println!("LIFTOFF!!!");
}
