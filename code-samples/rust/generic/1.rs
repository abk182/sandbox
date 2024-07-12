use std::ops::Add;

#[derive(Debug)]
struct Finger(i32);
#[derive(Debug)]
struct Hand(i32);

impl Add<Hand> for Finger {
    type Output = String;

    fn add(self, other: Hand) -> String {
        return String::from("PHHHHHHH");
    }
}

fn main() {
    println!("{:?}",Hand(4) + Finger(3));
}