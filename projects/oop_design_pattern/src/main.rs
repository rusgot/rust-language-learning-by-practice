use oop_design_pattern::Post;
use std::thread;
use std::time::Duration;

fn main() {
    let mut post = Post::new();
    post.add_text("My first post!");
    println!("Content: {}", post.content());

    post.request_review();
    println!("Content: {}", post.content());

    post.approve();
    println!("Content: {}", post.content());

    thread::sleep(Duration::from_millis(3000));

    post.approve();
    println!("Content: {}", post.content());

    post.reject();
}
