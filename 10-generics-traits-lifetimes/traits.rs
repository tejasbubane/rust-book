fn main() {
    let tweet = Tweet {
        username: String::from("ebooks"),
        content: String::from("you don't know everything - read them"),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}", tweet.summarize());

    let news = NewsArticle {
        headline: String::from("Markets soar"),
        location: String::from("Mumbai"),
        author: String::from("Tejas"),
        content: String::from("Markets on monday soared to a 52 week record high"),
    };
    println!("Breaking news: {}", news.summarize());

    notify(tweet);
}

trait Summary {
    fn summarize(&self) -> String;
}

struct Tweet {
    username: String,
    content: String,
    reply: bool,
    retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

trait SummaryWithDefault {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}

struct NewsArticle {
    headline: String,
    location: String,
    author: String,
    content: String,
}

// use default implementation
impl SummaryWithDefault for NewsArticle {}

fn notify(item: impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
