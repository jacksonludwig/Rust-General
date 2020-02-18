use std::fmt::{Display, Debug};

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());
}

pub trait Summary { // most like interface
    fn summarize(&self) -> String; // you may have a default body in the function which would be used if not overwritten
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
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
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub fn notify(item: impl Summary) { // accepts any type that implements the specified trait
    println!("Breaking news! {}", item.summarize());
}

pub fn notify_possibly_dif_types(item1: impl Summary, item2: impl Summary) {} // impl trait

pub fn notify_not_dif_types<T: Summary>(item1: T, item2: T) {} // trait bound

pub fn notify_multiple_impl(item: impl Summary + Display) {}

fn some_function<T: Display + Clone, U: Clone + Debug>(t: T, u: U) -> i32 {}
fn some_function_concise<T, U>(t: T, u: U) -> i32
    where T: Display + Clone,
          U: Clone + Debug
{
    // stuff
}

fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}