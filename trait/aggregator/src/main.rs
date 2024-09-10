use aggregator::{Summary, Tweet,NewsArticle, notify};

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };

    //println!("1 new tweet: {}", tweet.summarize_author());

    let news = NewsArticle {
        headline: "headline".to_string(),
        location: "location".to_string(),
        author: "Park".to_string(),
        content: "contents".to_string(),
    };

    //println!("1 new NewsArticle: {}", news.summarize());
    notify(&tweet);
    notify(&news);
}