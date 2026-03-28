pub fn fizz_buzz(n: i32) -> Vec<String> {
    // Implement your solution here
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_three() {
        assert_eq!(fizz_buzz(3), vec!["1", "2", "Fizz"]);
    }

    #[test]
    fn test_five() {
        assert_eq!(fizz_buzz(5), vec!["1", "2", "Fizz", "4", "Buzz"]);
    }

    #[test]
    fn test_fifteen() {
        assert_eq!(
            fizz_buzz(15),
            vec![
                "1", "2", "Fizz", "4", "Buzz", "Fizz", "7", "8", "Fizz", "Buzz", "11", "Fizz",
                "13", "14", "FizzBuzz"
            ]
        );
    }

    #[test]
    fn test_one() {
        assert_eq!(fizz_buzz(1), vec!["1"]);
    }

    #[test]
    fn test_fizzbuzz_at_30() {
        let result = fizz_buzz(30);
        assert_eq!(result[29], "FizzBuzz");
        assert_eq!(result[14], "FizzBuzz");
    }
}
