pub fn longest_palindrome(s: String) -> String {
    // Implement your solution here
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let result = longest_palindrome("babad".to_string());
        assert!(result == "bab" || result == "aba");
    }

    #[test]
    fn test_example_2() {
        assert_eq!(longest_palindrome("cbbd".to_string()), "bb");
    }

    #[test]
    fn test_single_char() {
        assert_eq!(longest_palindrome("a".to_string()), "a");
    }

    #[test]
    fn test_two_same_chars() {
        assert_eq!(longest_palindrome("aa".to_string()), "aa");
    }

    #[test]
    fn test_entire_string_palindrome() {
        assert_eq!(longest_palindrome("racecar".to_string()), "racecar");
    }

    #[test]
    fn test_no_palindrome_longer_than_one() {
        let result = longest_palindrome("abcd".to_string());
        assert_eq!(result.len(), 1);
    }

    #[test]
    fn test_even_length_palindrome() {
        assert_eq!(longest_palindrome("abccba".to_string()), "abccba");
    }

    #[test]
    fn test_palindrome_at_end() {
        let result = longest_palindrome("abacdc".to_string());
        assert!(result == "aba" || result == "cdc");
    }

    #[test]
    fn test_all_same_chars() {
        assert_eq!(longest_palindrome("aaaa".to_string()), "aaaa");
    }
}
