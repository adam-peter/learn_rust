use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let file_path = &args[2];
    println!("Searching for \"{query}\" in \"{file_path}\"");

    let contents = fs::read_to_string(file_path)
        .expect("Cannot read the file");
    println!("Contents:\n{contents}");
}
