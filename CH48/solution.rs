pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
    // Implement your solution here
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn to_string_vec(v: Vec<&str>) -> Vec<String> {
        v.into_iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn test_example_1() {
        assert_eq!(
            word_break("leetcode".to_string(), to_string_vec(vec!["leet", "code"])),
            true
        );
    }

    #[test]
    fn test_example_2() {
        assert_eq!(
            word_break(
                "applepenapple".to_string(),
                to_string_vec(vec!["apple", "pen"])
            ),
            true
        );
    }

    #[test]
    fn test_example_3() {
        assert_eq!(
            word_break(
                "catsandog".to_string(),
                to_string_vec(vec!["cats", "dog", "sand", "and", "cat"])
            ),
            false
        );
    }

    #[test]
    fn test_single_char() {
        assert_eq!(
            word_break("a".to_string(), to_string_vec(vec!["a"])),
            true
        );
    }

    #[test]
    fn test_single_char_missing() {
        assert_eq!(
            word_break("b".to_string(), to_string_vec(vec!["a"])),
            false
        );
    }

    #[test]
    fn test_repeated_word() {
        assert_eq!(
            word_break("aaaa".to_string(), to_string_vec(vec!["a"])),
            true
        );
    }

    #[test]
    fn test_overlapping_words() {
        assert_eq!(
            word_break(
                "catsanddog".to_string(),
                to_string_vec(vec!["cats", "dog", "sand", "and", "cat"])
            ),
            true
        );
    }

    #[test]
    fn test_no_segmentation() {
        assert_eq!(
            word_break("abcd".to_string(), to_string_vec(vec!["a", "abc", "b", "cd"])),
            true
        );
    }

    #[test]
    fn test_empty_impossible() {
        assert_eq!(
            word_break("hello".to_string(), to_string_vec(vec!["world"])),
            false
        );
    }

    #[test]
    fn test_whole_string_is_word() {
        assert_eq!(
            word_break(
                "butterfly".to_string(),
                to_string_vec(vec!["butterfly"])
            ),
            true
        );
    }
}
