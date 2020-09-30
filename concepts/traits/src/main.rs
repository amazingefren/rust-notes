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
fn main() {
    let tweet = Tweet {
        username: String::from("amazingefren"),
        content: String::from("The presidential debate was a disaster"),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}", tweet.default_summarize());
    println!("1 new tweet: {}", tweet.default_summarize());
    println!("1 new tweet: {}", tweet.read());
}
