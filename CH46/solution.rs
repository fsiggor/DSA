pub fn is_palindrome(s: String) -> bool {
    // Implement your solution here
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(is_palindrome("A man, a plan, a canal: Panama".to_string()), true);
    }

    #[test]
    fn test_example_2() {
        assert_eq!(is_palindrome("race a car".to_string()), false);
    }

    #[test]
    fn test_empty_after_cleanup() {
        assert_eq!(is_palindrome(" ".to_string()), true);
    }

    #[test]
    fn test_single_char() {
        assert_eq!(is_palindrome("a".to_string()), true);
    }

    #[test]
    fn test_with_numbers() {
        assert_eq!(is_palindrome("12321".to_string()), true);
    }

    #[test]
    fn test_not_palindrome() {
        assert_eq!(is_palindrome("hello".to_string()), false);
    }

    #[test]
    fn test_mixed_case() {
        assert_eq!(is_palindrome("AbBa".to_string()), true);
    }
}
