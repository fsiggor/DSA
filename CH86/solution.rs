pub fn is_palindrome(x: i32) -> bool {
    // Implement your solution here
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_palindrome() {
        assert_eq!(is_palindrome(121), true);
    }

    #[test]
    fn test_negative() {
        assert_eq!(is_palindrome(-121), false);
    }

    #[test]
    fn test_ends_with_zero() {
        assert_eq!(is_palindrome(10), false);
    }

    #[test]
    fn test_zero() {
        assert_eq!(is_palindrome(0), true);
    }

    #[test]
    fn test_single_digit() {
        assert_eq!(is_palindrome(7), true);
    }

    #[test]
    fn test_large_palindrome() {
        assert_eq!(is_palindrome(1234321), true);
    }

    #[test]
    fn test_even_digits_palindrome() {
        assert_eq!(is_palindrome(1221), true);
    }

    #[test]
    fn test_not_palindrome() {
        assert_eq!(is_palindrome(123), false);
    }
}
