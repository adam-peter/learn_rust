fn main() {
    let s = String::from("hello world");
    let hello: &str = &s[..5];
    let world: &str = &s[6..];
    let s2: &String = &s; // String reference, not a slice
    println!("{hello}, {world}, {s2}");

    let s = String::from("hello");
    let slice1 = &s[0..2];
    let slice2 = &s[..2];
    assert_eq!(slice1, slice2);

    let slice1 = &s[3..s.len()];
    let slice2 = &s[3..];
    assert_eq!(slice1, slice2);

    let slice1 = &s[0..s.len()];
    let slice2 = &s[..];
    assert_eq!(slice1, slice2);

    // string literals are slices
    let s = "Hello, world!"; // s is an immutable reference to the binary
    println!("{s}");

    // using first_word with its &str parameter
    let my_string = String::from("hello world");
    let word1 = first_word(&my_string[0..6]);
    let word2 = first_word(&my_string[..]);
    let word3 = first_word(&my_string);
    assert_eq!((word1, word2, word3), ("hello", "hello", "hello"));

    let my_string_literal = "hello world";
    let word1 = first_word(&my_string_literal[0..6]);
    let word2 = first_word(&my_string_literal[..]);
    let word3 = first_word(my_string_literal); // works on the string slice itself
    assert_eq!((word1, word2, word3), ("hello", "hello", "hello"));

    // other slices
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    assert_eq!(slice, &[2, 3]);

    println!(
        "&String={} &str={}",
        std::mem::size_of::<&String>(), // &String contains just the pointer to the stack (reference)
        std::mem::size_of::<&str>(), // &str contains the direct pointer to the heap and the size
    );
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}