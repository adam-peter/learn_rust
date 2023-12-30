use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    println!("{scores:#?}");

    let team_name = String::from("Blue");
    let team_score = scores.get(&team_name).copied().unwrap_or(-1);
    println!("{team_name}: {team_score}");

    // iterating over a hm happens in an arbitrary order
    for (key, value) in &scores {
        println!("== {key}: {value} ==");
    }

    let key = String::from("Favorite color");
    let value = String::from("Blue");
    let mut map = HashMap::new();
    map.insert(key, value);
    // println!("== {key}: {value} =="); - cannot use values after moving them to hashmap

    // Updating a hm
    // - overwriting the old value (insert)
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);
    println!("{scores:?}");

    // - add a new key-value pair if it doesn't exist
    // .entry() returns an Entry (may / may not exist)
    // .or_insert() Entry's method, inserts a value for the key if the Entry doesn't exist
    scores.entry(String::from("Blue")).or_insert(999);
    scores.entry(String::from("Green")).or_insert(30);
    println!("{scores:?}");

    // - updating the old value
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{map:#?}");

    let mut h: HashMap<char, Vec<usize>> = HashMap::new();
    for (i, c) in "hello!".chars().enumerate() { // store vectors of indexes of each letter
        h.entry(c).or_insert(Vec::new()).push(i);
    }
    let mut sum = 0;
    for i in h.get(&'l').unwrap() { // sum up the indexes of a letter (l)
        sum += *i;
    }
    println!("{}", sum); // prints 2 + 3 -> 5
}