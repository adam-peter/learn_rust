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
    let s1 = capitalize_first_letter(s1);
    let phrase = format!("{s1}, {s2}, {s3}, go!", );
    println!("{phrase}");

    let s1 = String::from("hello");
    // println!("{}", &s1[0]); - errors, cannot index into strings
    // instead of indexing into strings, create string slices
    // for operating on strings, it's a good idea to use chars or bytes:
    for c in "Зд".chars() {
        println!("Char: {c}");
    }
    for b in "Зд".bytes() {
        println!("Byte: {b}");
    }


}

// fn capitalize_first_letter(mut s: String) -> String {
//     let (first, second) = s.split_at_mut(1);
//     format!("{}{}", first.to_uppercase(), second)
// }

fn capitalize_first_letter(mut s: String) -> String {
    let first = &mut s[0..1];
    first.to_uppercase() + &s[1..]
}