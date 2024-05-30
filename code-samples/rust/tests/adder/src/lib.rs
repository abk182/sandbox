#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        let t = format!("{}", 'a');
        self.width > other.width && self.height > other.height
    }
}

pub fn add_two(a: i32) -> i32 {
    println!("a = {}", a);

    if a == 0 {
        panic!("a is 0")
    }

    a + 2
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { value }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller), "larger rect is smaller");
    }

    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2), "returned {}", add_two(2));
    }

    #[test]
    #[should_panic(expected = "a is 0")]
    fn it_panics() {
        add_two(0);
    }

    #[test]
    #[should_panic]
    fn greater_than_100() {
        Guess::new(200);
    }

    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}
