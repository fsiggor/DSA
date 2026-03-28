pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
    // Implement your solution here
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn to_vec(v: Vec<&str>) -> Vec<String> {
        v.into_iter().map(String::from).collect()
    }

    #[test]
    fn test_example_1() {
        assert_eq!(
            ladder_length(
                "hit".to_string(),
                "cog".to_string(),
                to_vec(vec!["hot", "dot", "dog", "lot", "log", "cog"])
            ),
            5
        );
    }

    #[test]
    fn test_no_path() {
        assert_eq!(
            ladder_length(
                "hit".to_string(),
                "cog".to_string(),
                to_vec(vec!["hot", "dot", "dog", "lot", "log"])
            ),
            0
        );
    }

    #[test]
    fn test_one_step() {
        assert_eq!(
            ladder_length("hot".to_string(), "dot".to_string(), to_vec(vec!["dot"])),
            2
        );
    }

    #[test]
    fn test_no_match() {
        assert_eq!(
            ladder_length("abc".to_string(), "xyz".to_string(), to_vec(vec!["xyz"])),
            0
        );
    }

    #[test]
    fn test_same_length_path() {
        assert_eq!(
            ladder_length(
                "a".to_string(),
                "c".to_string(),
                to_vec(vec!["a", "b", "c"])
            ),
            2
        );
    }

    #[test]
    fn test_end_not_in_list() {
        assert_eq!(
            ladder_length("hit".to_string(), "cog".to_string(), to_vec(vec!["hot"])),
            0
        );
    }
}
