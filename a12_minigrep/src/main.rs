use std::{env, process};
use a12_minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    println!("Searching for \"{}\" in \"{}\"", config.query, config.file_path);

    if let Err(e) = a12_minigrep::run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}