use std::fmt::Display;
pub mod largest;
pub struct Point<T, U> {
    pub x: T,
    pub y: U,
}
impl<T, U> Point<T, U> {
    pub fn mixup<V, W> (self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

pub trait Summary {
    fn summarize(&self) -> String;
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
        format!("{} : {}", self.username, self.content)
    }
}

pub fn notify(item: impl Summary) {
    println!("Break news: {}", item.summarize());
}

pub struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    pub fn new(x: T, y: T) -> Self {
        Pair {
            x,
            y,
        }
    }
}
impl<T: Display + PartialOrd> Pair<T> {
    pub fn cmp_display(&self) {
        if self.x > self.y {
            println!("The largest is x : {}", self.x);
        } else {
            println!("The largest is y : {}", self.y);
        }
    }
}

pub struct ImportantExpert<'a> {
    pub part: &'a str,
}
impl<'a> ImportantExpert<'a> {
    pub fn level(&'a self) -> i32 {
        3
    }
    pub fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("announcement is {}", announcement);
        self.part
    }
}