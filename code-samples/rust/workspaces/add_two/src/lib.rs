use rand;

pub fn add_one(x: i32) -> i32 {
    x + 2
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(4, add_one(2));
    }
}