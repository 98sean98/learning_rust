mod blog {
    pub struct Post {
        content: String
    }

    pub struct DraftPost {
        content: String
    }

    pub struct PendingFirstReviewPost {
        content: String,
    }

    pub struct PendingSecondReviewPost {
        content: String,
    }

    impl Post {
        pub fn new() -> DraftPost {
            DraftPost {
                content: String::new()
            }
        }

        pub fn content(&self) -> &str {
            &self.content
        }
    }

    impl DraftPost {
        pub fn add_text(&mut self, text: &str) {
            self.content.push_str(text);
        }

        pub fn request_review(self) -> PendingFirstReviewPost {
            PendingFirstReviewPost {content: self.content}
        }
    }

    impl PendingFirstReviewPost {
        pub fn approve(self) -> PendingSecondReviewPost {
            PendingSecondReviewPost {content: self.content}
        }

        pub fn reject(self) -> DraftPost {
            DraftPost {content: self.content}
        }
    }

    impl PendingSecondReviewPost {
        pub fn approve(self) -> Post {
            Post {content: self.content}
        }

        pub fn reject(self) -> DraftPost {
            DraftPost {content: self.content}
        }
    }
}


use blog::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");

    let post = post.request_review();

    let post = post.approve();
    let post = post.approve();

    assert_eq!("I ate a salad for lunch today", post.content());
}
