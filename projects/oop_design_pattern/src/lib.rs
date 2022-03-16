pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }

    pub fn add_text(&mut self, text: &str) {
        let obj = self.state.as_mut().unwrap();
        let val = obj.add_text("hello");
        self.content.push_str("hello");
    }

    pub fn content(&self) -> &str {
        // We call the as_ref method on the Option because we want a reference to the value inside the Option rather than ownership of the value.
        // Because state is an Option<Box<dyn State>>, when we call as_ref, an Option<&Box<dyn State>> is returned.
        // If we didn’t call as_ref, we would get an error because we can’t move state out of the borrowed &self of the function parameter.
        self.state.as_ref().unwrap().content(self) // self here is the post instance
    }

    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review());
        }
    }

    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve());
        }
    }

    pub fn reject(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.reject());
        }
    }
}

trait State {
    // Note that rather than having self, &self, or &mut self as the first parameter of the method, we have self: Box<Self>.
    // This syntax means the method is only valid when called on a Box holding the type. This syntax takes ownership of Box<Self>,
    // invalidating the old state so the state value of the Post can transform into a new state.
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        ""
    }
    fn reject(self: Box<Self>) -> Box<dyn State>;
    fn add_text(self: Box<Self>, text: &str) -> &str;
}

#[derive(Debug, Copy, Clone)]
struct Draft {}

impl State for Draft {
    fn add_text(self: Box<Self>, text: &str) -> &str {
        text
    }

    fn request_review(self: Box<Self>) -> Box<dyn State> {
        println!("Review requested on {:?}", self);
        Box::new(PendingReview { approval_count: 0 })
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        println!("Cannot approve a post in Draft state");
        self
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        println!("Cannot reject a post in draft state!");
        self
    }
}

#[derive(Debug, Clone, Copy)]
struct PendingReview {
    approval_count: u8,
}

impl State for PendingReview {
    fn add_text(self: Box<Self>, text: &str) -> &str {
        println!("Cannot add text to a post not in draft state.");
        ""
    }

    fn request_review(self: Box<Self>) -> Box<dyn State> {
        println!("Review requested on {:?}", self);
        self
    }

    fn approve(mut self: Box<Self>) -> Box<dyn State> {
        self.approval_count = self.approval_count + 1;
        match self.approval_count {
            1 => {
                println!("1 more approval required to publish...");
                return Box::new(PendingReview {
                    approval_count: self.approval_count,
                });
            }
            2 => {
                println!("Post approved!");
                return Box::new(Published {});
            }
            _ => panic!("Shouldn't happen"),
        }
        println!("Post approved!");
        Box::new(Published {})
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        println!("Post rejected...");
        Box::new(Draft {})
    }
}

#[derive(Debug, Clone, Copy)]
struct Published {}

impl State for Published {
    fn add_text(self: Box<Self>, text: &str) -> &str {
        println!("Cannot add text to a post not in draft state.");
        ""
    }

    fn request_review(self: Box<Self>) -> Box<dyn State> {
        println!("Post already reviewed and approved.");
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        println!("Post already approved");
        self
    }

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        println!("Cannot reject an already published post!");
        self
    }
}
