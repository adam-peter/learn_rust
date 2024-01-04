use a10_generics::{Summary, Tweet};

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("Of course, as you probably already know, people"),
        reply: false,
        retweet: false
    };
    println!("1 new tweet: {}", tweet.summarize());
}
