use std::{collections::HashMap, io};

fn main() {
    let mut employees: HashMap<String, Vec<String>> = HashMap::new();
    println!("EMPLOYEE MANAGEMENT\n\
    Welcome! This is an employee management system CLI app.\n\
    - quit -> quit the app\n\
    - print -> print the employees\n\
    - add {{name}} to {{team}} -> adds the employee to the specified team");

    loop {
        let mut command = String::new();
        println!("\nInput your command:");
        io::stdin().read_line(&mut command).expect("Failed to read line");
        command = command.trim().to_lowercase().to_string();
        match command.as_str() {
            "quit" => break,
            "print" => println!("\nEmployees: {employees:#?}"),
            _ => {
                let words: Vec<&str> = command.split_whitespace().collect();
                if words.len() != 4 {
                    println!("Enter a valid command (\"Add x to y\")");
                    continue;
                }

                let employees_vec = employees.entry(words[3].to_string()).or_insert(Vec::new());
                employees_vec.push(words[1].to_string());
            }
        }
    }
    println!("\nGOODBYE ^^");
}

fn get_median(v: &Vec<i32>) -> f64 {
    let mut v = v.clone();
    v.sort();
    let len = v.len();

    if len % 2 == 0 {
        let one = v[len / 2];
        let two = v[(len / 2) - 1];
        (one + two) as f64 / 2.
    } else {
        v[len / 2] as f64
    }
}

fn get_mode(v: &Vec<i32>) -> i32 {
    let mut map = HashMap::new();
    for num in v {
        let entry = map.entry(num).or_insert(0);
        *entry += 1
    }

    let mut max = *map.keys().next().unwrap();

    for (key, value) in &map {
        if map[key] > map[&max] {
            max = *key;
        }
    }
    *max
}

fn convert_text(text: &str) -> String {
    let text = text.to_lowercase();
    let mut new_text = Vec::new();
    let words = text.split_whitespace();
    for word in words {
        let pig_word = convert_word(word);
        new_text.push(pig_word);
    }

    new_text.join(" ")
}

fn convert_word(word: &str) -> String {
    let consonants = String::from("bcdfghjklmnpqrstvwxz");

    if word.starts_with(|letter| {
        consonants.contains(letter)
    }) {
        let (w1, w2) = word.split_at(1);
        format!("{}-{}ay", w2, w1)
    } else {
        format!("{word}-hay")
    }
}
