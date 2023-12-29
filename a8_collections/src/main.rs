fn main() {
    let mut s = String::new();
    let data = "content";
    let s = data.to_string();
    let s = "content".to_string();
    let s = String::from("content");

    let mut s = String::from("foo");
    s.push_str("bar");
    println!("{s}");
    let mut s = String::from("lo");
    s.push('l'); // push pushes a character
    println!("{s}");

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // plus operator takes ownership of 1 and reference of 2
    println!("{}", s3);
    // better way to combine more strings - the format!() macro
    let (s1, s2, s3) = (String::from("rock"), String::from("paper"), String::from("scissors"));
    let s1 = capitalize_first_letter(&s1);
    let phrase = format!("{s1}, {s2}, {s3}, go!", );
    println!("{phrase}");
}

fn capitalize_first_letter(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        Some(first_char) => {
            let capitalized_first_char = first_char.to_uppercase().collect::<String>();
            capitalized_first_char + chars.as_str()
        },
        None => String::new()
    }
}