use std::fmt::Display;

fn main() {
    // lifetimes = type of generic; don't ensure behavior, they ensure that the parameter is valid
    let s1 = String::from("abcd");
    let s2 = "xyz";
    let result = longest(&s1, s2);
    println!("The longest string is {result}");

    // lifetime annotations
    // &i32 - reference
    // &'a i32 - ref with a lifetime
    // &'a mut i32 - mut ref with a lifetime

    let s1 = String::from("long string");
    {
        let s2 = String::from("short");
        let result = longest(&s1, &s2);
        println!("Longer string: {}", result);
    }
    // println!("Longer string: {}", result); - result doesn't live long enough

    // structs with references
    #[derive(Debug)]
    struct ImportantExcerpt<'a> {
        part: &'a str,
    }
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence
    };
    println!("{i:?}");

    impl<'a> ImportantExcerpt<'a> {
        fn level(&self) -> i32 {
            3
        }

        fn announce_and_return_part(&self, announcement: &str) -> &str { // output type has lifetime of self - 3rd elision rule
            println!("Attention please: {announcement}");
            self.part
        }
    }

    // static lifetimes - can live for the entirety of the program
    let s: &'static str = "I have a static lifetime.";
}

fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}

fn first<'a>(first: &'a str, second: &str) -> &'a str { // second doesn't have to have a lifetime
    first
}

// lifetime elision - simple example of borrow checker inferring life annotations
fn first_word(s: &str) -> &str {
    for (i, &item) in s.as_bytes().iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}

// generics, traits, trait bounds, lifetimes - together!
fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
    where
        T: Display
{
    println!("Announcement! {ann}");
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

struct Foo<'a> {
    bar: &'a i32,
}

// fn baz(f: Foo) -> &i32 {
//     f.bar
// }
// Will compile to:
fn baz<'a>(f: Foo<'a>) -> &'a i32 {
    f.bar
}

// fn baz2(f: &Foo) -> &i32 {
//     f.bar
// }
// Will compile to:
fn baz2<'a>(f: &'a Foo<'a>) -> &'a i32 {
    f.bar
}






