// Traits - define functionality of a type that implements it (similar to interfaces)
// - we can use Trait bounds to specify a generic type must implement a certain trait - must have certain behavior

pub trait Summary {
    fn summarize(&self) -> String;
}

// implementing a trait on a type

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String
}
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}