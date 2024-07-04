pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
    approves: i32,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
            approves: 0,
        }
    }

    pub fn get_state(&mut self) -> String {
        self.state.as_ref().unwrap().get_state()
    }

    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn request_review(&mut self) {
        // let mut a: Option<i32> = Some(3);

        // println!("{:?}", a.take());
        // println!("{:?}", a.take());

        // if let Some(s) = a.take() {
        //     println!("{}", s);
        // }

        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
    }

    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }

    pub fn reject(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.reject())
        }
    }

    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(self)
    }
}

trait State {
    fn get_state(&self) -> String {
        String::from("unknown")
    }
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn reject(self: Box<Self>) -> Box<dyn State>;
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        ""
    }
}

struct Draft {}

impl State for Draft {
    fn get_state(&self) -> String {
        String::from("draft")
    }
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview { approves: 0 })
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

struct PendingReview {
    approves: i32,
}

impl State for PendingReview {
    fn get_state(&self) -> String {
        String::from("pending-review")
    }
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn approve(mut self: Box<Self>) -> Box<dyn State> {
        if (self.approves < 2) {
            self.approves += 1;
            self
        } else {
            Box::new(Published {})
        }
    }
    fn reject(self: Box<Self>) -> Box<dyn State> {
        Box::new(Draft {})
    }
}

struct Published {}

impl State for Published {
    fn get_state(&self) -> String {
        String::from("published")
    }
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}
