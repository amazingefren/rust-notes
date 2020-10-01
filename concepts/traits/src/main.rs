pub trait Summary {
    fn read(&self) -> String;
    fn summarize_author(&self) -> String;
    fn default_summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn read(&self) -> String {
        format!("{}, by {}, ({})", self.headline, self.author, self.location)
    }
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

pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.default_summarize())
}

//Trait Bound
pub fn notify_bound<T: Summary>(item: &T) {
    println!("Breaking News! {}", item.default_summarize())
}

impl Summary for Tweet {
    fn read(&self) -> String {
        format!("@{}: {}", self.username, self.content)
    }
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

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
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

fn main() {
    let tweet = Tweet {
        username: String::from("amazingefren"),
        content: String::from("The presidential debate was a disaster"),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}", tweet.default_summarize());
    println!("1 new tweet: {}", tweet.read());

    let pair = Pair::new(5, 10);
    pair.cmp_display();
    {
        let r;
        {
            let x = 5;
            r = x
            // Here i am giving ownership of x to r
            // &x is not used here as
        } // x will be invalidated here, assigning a ptr to a ptr that doesn't exist doesn't work
        println!("r: {}", r);
    }
    {
        let x = 5;
        let r = &x; // here there are to variables in the stack, the stack for x is never invalidated.
                    // so assigning the ptr or 'r' to ptr of 'x' will work as it never gets invalidated
        println!("r: {}", r); // so here to we can ref r, which points to ptr of 'x' pointing to heap of x being 5
        let word1 = "Hello";
        let word2 = "World!";
        let longest_word = longest(word1, word2);
        println!("Longest Word: {}", longest_word);
    }
}

// Lifetime annotations in Function
/*
    example types:
    &i32 - explicit
    &'a i32 - explicit lifetime
    &'a mut i32 - mutable explicit lifetime
    ( 'a ) is setting a explicit lifetime
*/
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
