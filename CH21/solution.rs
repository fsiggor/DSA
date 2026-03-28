pub fn is_anagram(s: String, t: String) -> bool {
    // Implement your solution here
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_anagram() {
        assert_eq!(is_anagram("anagram".to_string(), "nagaram".to_string()), true);
    }

    #[test]
    fn test_not_anagram() {
        assert_eq!(is_anagram("rat".to_string(), "car".to_string()), false);
    }

    #[test]
    fn test_different_lengths() {
        assert_eq!(is_anagram("ab".to_string(), "abc".to_string()), false);
    }

    #[test]
    fn test_single_char_same() {
        assert_eq!(is_anagram("a".to_string(), "a".to_string()), true);
    }

    #[test]
    fn test_single_char_different() {
        assert_eq!(is_anagram("a".to_string(), "b".to_string()), false);
    }

    #[test]
    fn test_repeated_chars() {
        assert_eq!(is_anagram("aabb".to_string(), "bbaa".to_string()), true);
    }

    #[test]
    fn test_same_chars_different_count() {
        assert_eq!(is_anagram("aaab".to_string(), "aabb".to_string()), false);
    }
}
