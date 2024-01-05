// Traits - define functionality of a type that implements it (similar to interfaces)
// - we can use Trait bounds to specify a generic type must implement a certain trait - must have certain behavior

use std::fmt::{Debug, Display};

pub trait Summary {
    // fn summarize(&self) -> String; - only define the method that must be implemented
    fn summarize(&self) -> String {
        format!("Read more from {}...", self.summarize_author())
    }

    fn summarize_author(&self) -> String;
}

// implementing a trait on a type

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

// impl Summary for NewsArticle {
//     fn summarize(&self) -> String {
//         format!("{}, by {} ({})", self.headline, self.author, self.location)
//     }
// }
impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("{}", self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

// using traits as function parameters
// pub fn notify(item: &impl Summary) { // takes a reference to an item that implements the Summary trait
//     println!("Breaking news! {}", item.summarize());
// }

// trait bounds - generic types bound by traits
pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// difference:
// pub fn notify(item1: &impl Summary, item2: &impl Summary) {} - parameters can be 2 different types implementing Summary
// pub fn notify<T: Summary>(item1: &T, item2: &T) {} - parameters must be of the same type, that implements Summary

// specify multiple trait bounds - "+" syntax
// pub fn notify(item: &(impl Summary + Display)) {}
// pub fn notify<T: Summary + Display>(item: &T) {}

// clearer trait bounds - move them in the "where" clause
// fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {}
fn some_function<T, U>(t: &T, u: &U) -> i32
    where
        T: Display + Clone,
        U: Clone + Debug
{
    0
}

// returning trait-bound types
fn return_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}
// useful when returning an iterator over something - the types are long and complex, instead return "impl Iterator"

// using trait bounds to conditionally implement methods
struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    // all Pair types implement new
    fn new(x: T, y: T) -> Self {
        Self { x, y } // Self = Pair<T> from the impl block
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    // only Pair types that implement Display & PartialOrd implement the cmp_display method
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

// conditional implementation of a trait on a type that implements a specific trait
// impl<T: Display> ToString for T {} - in the std library - all Display types should implement the ToString trait
