use post::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    println!("{}", post.get_state());
    assert_eq!("", post.content());

    post.request_review();
    println!("{}", post.get_state());
    assert_eq!("", post.content());

    post.reject();
    println!("{}", post.get_state());
    assert_eq!("", post.content());

    post.request_review();
    println!("{}", post.get_state());
    assert_eq!("", post.content());

    post.approve();
    println!("{}", post.get_state());
    post.approve();
    println!("{}", post.get_state());
    post.approve();
    println!("{}", post.get_state());
    assert_eq!("I ate a salad for lunch today", post.content());
}