use std::collections::HashMap;
use rand::Rng;

// use std::cmp::Ordering;
// use std::io;
// use std::{cmp::Ordering, io}; - same thing

// use std::io;
// use std::io::Write;
use std::io::{self, Write}; // - same thing

use std::collections::*; // all public items - used with testing, in the tests module

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
    println!("Map: {map:#?}");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("Secret number is {secret_number}");
}