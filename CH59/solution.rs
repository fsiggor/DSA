pub fn my_atoi(s: String) -> i32 {
    // Implement your solution here
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(my_atoi("42".to_string()), 42);
    }

    #[test]
    fn test_example_2() {
        assert_eq!(my_atoi("   -42".to_string()), -42);
    }

    #[test]
    fn test_example_3() {
        assert_eq!(my_atoi("4193 with words".to_string()), 4193);
    }

    #[test]
    fn test_example_4() {
        assert_eq!(my_atoi("words and 987".to_string()), 0);
    }

    #[test]
    fn test_empty_string() {
        assert_eq!(my_atoi("".to_string()), 0);
    }

    #[test]
    fn test_only_whitespace() {
        assert_eq!(my_atoi("   ".to_string()), 0);
    }

    #[test]
    fn test_positive_sign() {
        assert_eq!(my_atoi("+1".to_string()), 1);
    }

    #[test]
    fn test_overflow_positive() {
        assert_eq!(my_atoi("2147483648".to_string()), 2147483647);
    }

    #[test]
    fn test_overflow_negative() {
        assert_eq!(my_atoi("-2147483649".to_string()), -2147483648);
    }

    #[test]
    fn test_leading_zeros() {
        assert_eq!(my_atoi("0032".to_string()), 32);
    }

    #[test]
    fn test_just_sign() {
        assert_eq!(my_atoi("-".to_string()), 0);
    }

    #[test]
    fn test_sign_with_spaces_after() {
        assert_eq!(my_atoi("- 42".to_string()), 0);
    }
}
