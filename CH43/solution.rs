pub fn str_str(haystack: String, needle: String) -> i32 {
    // Implement your solution here
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(str_str("sadbutsad".to_string(), "sad".to_string()), 0);
    }

    #[test]
    fn test_example_2() {
        assert_eq!(str_str("leetcode".to_string(), "leeto".to_string()), -1);
    }

    #[test]
    fn test_example_3() {
        assert_eq!(str_str("hello".to_string(), "ll".to_string()), 2);
    }

    #[test]
    fn test_needle_at_end() {
        assert_eq!(str_str("abcdef".to_string(), "def".to_string()), 3);
    }

    #[test]
    fn test_needle_equals_haystack() {
        assert_eq!(str_str("abc".to_string(), "abc".to_string()), 0);
    }

    #[test]
    fn test_needle_longer_than_haystack() {
        assert_eq!(str_str("ab".to_string(), "abc".to_string()), -1);
    }

    #[test]
    fn test_single_char_found() {
        assert_eq!(str_str("a".to_string(), "a".to_string()), 0);
    }

    #[test]
    fn test_single_char_not_found() {
        assert_eq!(str_str("a".to_string(), "b".to_string()), -1);
    }

    #[test]
    fn test_repeated_pattern() {
        assert_eq!(str_str("aabaabaaf".to_string(), "aabaaf".to_string()), 3);
    }
}
