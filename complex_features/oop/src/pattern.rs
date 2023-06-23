mod blog {
    pub struct Post {
        state: Option<Box<dyn State>>,
        content: String,
    }

    impl Post {
        pub fn new() -> Post {
            Post {
                state: Some(Box::new(Draft {})),
                content: String::new()
            }
        }

        pub fn add_text(&mut self, text: &str) {
            let text = self.state.as_ref().unwrap().add_text(text);
            self.content.push_str(text);
        }

        pub fn content(&self) -> &str {
            // use an internal .content() method depending on the state
            self.state
                .as_ref()
                .unwrap()
                .content(self)
        }

        pub fn request_review(&mut self) {
            // only call the request_review method of a trait object
            // .take() takes the value out of the Option, leaving a None in its place
            // that means self.state immediately becomes None after .take()
            // important to take the value out to transfer ownership to s.request_review() method
            if let Some(s) = self.state.take() {
                self.state = Some(s.request_review());
            }

            // this approach ensures that the Post cannot access it's old state value
            // each State is responsible for its own rules
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

    // define the behavior shared by different Post states
    trait State {
        fn add_text<'a>(&self, _text: &'a str) -> &'a str {
            ""
        }

        fn request_review(self: Box<Self>) -> Box<dyn State>;
        // take ownership of old state
        // return new state

        fn approve(self: Box<Self>) -> Box<dyn State>;

        // annotate lifetime `'a` to tell the compiler that the returned value has the same lifetime as the Post
        fn content<'a>(&self, _post: &'a Post) -> &'a str {
            ""
        }

        fn reject(self: Box<Self>) -> Box<dyn State>;
    }

    struct Draft {}

    impl State for Draft {
        fn add_text<'a>(&self, text: &'a str) -> &'a str {
            text
        }

        fn request_review(self: Box<Self>) -> Box<dyn State> {
            Box::new(PendingReview::new())
        }

        fn approve(self: Box<Self>) -> Box<dyn State> {
            self
        }

        fn reject(self: Box<Self>) -> Box<dyn State> {
            self
        }
    }

    struct PendingReview {
        approval_count: std::cell::RefCell<u32>,
    }

    impl PendingReview {
        fn new() -> Self {
            PendingReview {approval_count: std::cell::RefCell::new(0)}
        }
    }

    impl State for PendingReview {
        fn request_review(self: Box<Self>) -> Box<dyn State> {
            self
        }

        fn approve(self: Box<Self>) -> Box<dyn State> {
            if *self.approval_count.borrow() == 1 {
                return Box::new(Published {});
            }
            *self.approval_count.borrow_mut() += 1;
            self
        }

        fn reject(self: Box<Self>) -> Box<dyn State> {
            Box::new(Draft {})
        }
    }

    struct Published {}

    impl State for Published {
        fn request_review(self: Box<Self>) -> Box<dyn State> {
            self
        }

        fn approve(self: Box<Self>) -> Box<dyn State> {
            self
        }

        // override default trait method
        fn content<'a>(&self, post: &'a Post) -> &'a str {
            &post.content
        }

        fn reject(self: Box<Self>) -> Box<dyn State> {
            self
        }
    }

}


use blog::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.add_text("This should not appear");
    post.reject();
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());

    // tbh, kinda bothersome
    // if a post is at a Draft state, outside code can't really tell, and will try to call .content()
    // which returns nothing
    // should make use of rust's compiler type system
    // to check if a particular method is even allowed during a certain state of post
}