fn main() {
    let mut post = oop_design_pattern::Post::new();
    post.add_text("My first post!");
    post.content();
    post.request_review();
}
