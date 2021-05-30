#![allow(unused)]
use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmd_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x: {}", self.x);
        } else {
            println!("The largest member is y: {}", self.y);
        }
    }
}

pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String{
        format!("{}: {}", self.username, self.content)
    }
}

pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

pub fn notify_verbose<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

fn some_function<T: Summary + Clone, U: Clone + Ord>(t: &T, u: &U) -> i32 {
    1
}

fn some_function2<T, U>(t: &T, u: &U) -> i32
    where T: Summary + Clone,
          U: Clone + Ord
{
    2
}

fn return_summarizable(switch: bool) -> impl Summary {
    // if swicth {
        NewsArticle {
            headline: String::from(
                "What every programmer should know about memory"
            ),
            location: String::from("Boston, MA, USA"),
            author: String::from("Ulrich Drepper"),
            content: String::from("Too long to past here lol..."),
        }
    // } else {
    /*
        Tweet {
            username: String::from("yaspr"),
            content: String::from(
                "Please. Read the fucking manual ffs..."
            ),
            reply: false,
            retweet: false,
        }
    }
    */
}

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let tweet = Tweet {
        username: String::from("yaspr"),
        content: String::from(
            "Please. Read the fucking manual ffs..."
        ),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());
    notify(&tweet);

    let article = NewsArticle {
        headline: String::from("What every programmer should know about memory"),
        location: String::from("Boston, MA, USA"),
        author: String::from("Ulrich Drepper"),
        content: String::from("Too long to past here lol..."),
    };

    println!("New article available! {}", article.summarize());
    notify_verbose(&article);

    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
}
