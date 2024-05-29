pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn try_test() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn try_panic() {
        panic!("Make this test fail");
    }
}
