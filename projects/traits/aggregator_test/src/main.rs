// extern crate aggregator;
use aggregator::code::{pls_work, NewsArticle, Summary, Tweet};
use aggregator::test_fn;
use aggregator_test::game::{player, welcome};

fn main() {
    let article = NewsArticle {
        headline: String::from("A Wild Degen Appears"),
        location: String::from("DegenVille"),
        author: String::from("Wojak"),
        content: String::from("This is Wojak reporting live from DegenVille. It seems a group of apes have appears out of the woodwork..."),
    };

    let tweet = Tweet {
        username: String::from("jakerumbles"),
        content: String::from("So bullish it hurts"),
        reply: false,
        retweet: true,
    };

    let tweet2 = Tweet::new(
        String::from("jakerumbles"),
        String::from("Buy the dip"),
        true,
        false,
    );

    println!("{}", article.summarize());
    println!("{}", tweet.summarize());
    println!("{}", tweet2.summarize());
    pls_work("hello");
    test_fn();

    player::hello(String::from("Jake"));
    welcome();
}
