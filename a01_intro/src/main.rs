use std::collections::HashMap;

fn main() {
    let nums: [i32; 3] = [1, 2, 3];
    for i in nums {
        println!("{i}");
    }
    println!("{}", &nums[1]);

    let mut mutable_arr = [1, 2, 0];
    mutable_arr[2] = 999;
    println!("{mutable_arr:?}");

    let zeros = [0; 100];
    println!("{:?}", &zeros[5..8]);

    let mut profile = HashMap::new();
    profile.insert("name", CharacterValue::Name(String::from("Adam")));
    profile.insert("age", CharacterValue::Age(18));
    profile.insert("items", CharacterValue::Items(vec![
        String::from("laptop"),
        String::from("book"),
        String::from("coat"),
    ]));
    println!("{profile:?}");
}

#[derive(Debug)]
enum CharacterValue {
    Name(String),
    Age(i32),
    Items(Vec<String>),
}