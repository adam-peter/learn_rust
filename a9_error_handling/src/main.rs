use std::fs::File;
use std::io::{self, ErrorKind, Read};
use std::error::Error; // Error is a trait

fn main() -> Result<(), Box<dyn Error>> { // Box<dyn Error>> - trait object, == any kind of error
    let file_result = File::open("hello.txt");
    let file = match file_result {
        Ok(f) => f,
        Err(e) => match e.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(f) => f,
                Err(e) => panic!("Problem creating the file: {e}")
            },
            other_error => panic!("Problem opening the file: {other_error:?}")
        }
    };
    println!("{file:?}");

    // using unwrap_or_else
    let file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {error:?}");
            })
        } else {
            panic!("Problem opening the file: {error:?}");
        }
    });
    println!("{file:?}");

    // shortcut with unwrap
    let file = File::open("hello.txt").unwrap();
    println!("{file:?}");

    // shortcut with expect
    let file = File::open("hello.txt").expect("hello.txt should be included in this project");
    println!("{file:?}");

    // let file = File::open("hello.txt")?; - error, cannot use "?" in a function that returns "()"
    // main can also return Result<(), Box<dyn Error>> - so we can use the "?" propagation operator
    let file = File::open("hello.txt")?;
    println!("{file:?}");
    Ok(())

    // code validation with custom types
    pub struct Guess {
        value: i32,
    }
    impl Guess {
        pub fn new(value: i32) -> Guess {
            if value > 100 || value < 1 {
                panic!("The value must be between 1 and 100, got {value}");
            }
            Guess {
                value
            }
        }

        pub fn value(&self) -> i32 {
            self.value
        }
    }
    // this is better than runtime manual checks everywhere in the program - you check only once and you can be sure the value is in the expected format at all times
}

// propagating errors
fn read_username_from_file(path: &str) -> Result<String, io::Error> {
    let file_result = File::open(path);
    let mut file = match file_result {
        Ok(f) => f,
        Err(e) => return Err(e)
    };

    let mut username = String::new();
    match file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e)
    }
}

// ? - shortcut for propagating errors
// - can only be used inside functions with return type Result<T, E>
fn read_username_from_file_2(path: &str) -> Result<String, io::Error> {
    let mut file = File::open(path)?; // this '?' either unwraps the value, or returns an error right away

    let mut username = String::new();
    file.read_to_string(&mut username)?;
    Ok(username)
}

// chaining propagation shortcuts
fn read_username_from_file_3(path: &str) -> Result<String, io::Error> {
    let mut username = String::new();
    File::open(path)?.read_to_string(&mut username)?;
    Ok(username)
}

// shorter way - fs::read_to_string(path).unwrap();

// the "?" operator can be used with functions returning Option<T>
fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last() // next returns an Option - we propagate its None value if present, otherwise unwrap
}