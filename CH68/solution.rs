pub fn length_of_longest_substring(s: String) -> i32 {
    // Implement your solution here
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(length_of_longest_substring("abcabcbb".to_string()), 3);
    }

    #[test]
    fn test_example_2() {
        assert_eq!(length_of_longest_substring("bbbbb".to_string()), 1);
    }

    #[test]
    fn test_example_3() {
        assert_eq!(length_of_longest_substring("pwwkew".to_string()), 3);
    }

    #[test]
    fn test_empty_string() {
        assert_eq!(length_of_longest_substring("".to_string()), 0);
    }

    #[test]
    fn test_single_char() {
        assert_eq!(length_of_longest_substring("a".to_string()), 1);
    }

    #[test]
    fn test_all_unique() {
        assert_eq!(length_of_longest_substring("abcdef".to_string()), 6);
    }

    #[test]
    fn test_with_spaces() {
        assert_eq!(length_of_longest_substring("ab cd".to_string()), 5);
    }

    #[test]
    fn test_two_chars_repeating() {
        assert_eq!(length_of_longest_substring("abba".to_string()), 2);
    }

    #[test]
    fn test_digits_and_letters() {
        assert_eq!(length_of_longest_substring("a1b2c3".to_string()), 6);
    }
}
