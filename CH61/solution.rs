pub fn min_window(s: String, t: String) -> String {
    // Implement your solution here
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(
            min_window("ADOBECODEBANC".to_string(), "ABC".to_string()),
            "BANC"
        );
    }

    #[test]
    fn test_example_2() {
        assert_eq!(
            min_window("a".to_string(), "a".to_string()),
            "a"
        );
    }

    #[test]
    fn test_example_3() {
        assert_eq!(
            min_window("a".to_string(), "aa".to_string()),
            ""
        );
    }

    #[test]
    fn test_no_match() {
        assert_eq!(
            min_window("abc".to_string(), "xyz".to_string()),
            ""
        );
    }

    #[test]
    fn test_t_equals_s() {
        assert_eq!(
            min_window("abc".to_string(), "abc".to_string()),
            "abc"
        );
    }

    #[test]
    fn test_duplicate_chars_in_t() {
        assert_eq!(
            min_window("aabbc".to_string(), "aab".to_string()),
            "aab"
        );
    }

    #[test]
    fn test_window_at_end() {
        assert_eq!(
            min_window("xxxxxabc".to_string(), "abc".to_string()),
            "abc"
        );
    }

    #[test]
    fn test_single_char_match() {
        assert_eq!(
            min_window("b".to_string(), "b".to_string()),
            "b"
        );
    }

    #[test]
    fn test_case_sensitive() {
        assert_eq!(
            min_window("Aa".to_string(), "a".to_string()),
            "a"
        );
    }
}
