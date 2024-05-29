use std::fmt::Display;

struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &'a str) -> &'a str {
        println!("Attention please: {}", announcement);
        announcement
    }
}


fn main() {
    let mut novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };

    println!("{}", i.part);

    let str1 = String::from("str1");
    let str2 = String::from("str2");

    to_tuple(&str1[..], &str2[..]);

    let s: &'static str = "I have a static lifetime.";

    let longest = longest_with_an_announcement("aaa", "bbbbbbb", "AAA");
    println!("{}", longest)
}

fn to_tuple<'a, 'b>(x: &'a str, y:&'b str) -> (&'a str, &'b str) {
    (x, y)
}

fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}