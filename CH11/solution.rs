pub fn longest_common_prefix(strs: Vec<String>) -> String {
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
            longest_common_prefix(to_vec(vec!["flower", "flow", "flight"])),
            "fl"
        );
    }

    #[test]
    fn test_example_2() {
        assert_eq!(
            longest_common_prefix(to_vec(vec!["dog", "racecar", "car"])),
            ""
        );
    }

    #[test]
    fn test_single_string() {
        assert_eq!(
            longest_common_prefix(to_vec(vec!["alone"])),
            "alone"
        );
    }

    #[test]
    fn test_identical_strings() {
        assert_eq!(
            longest_common_prefix(to_vec(vec!["test", "test", "test"])),
            "test"
        );
    }

    #[test]
    fn test_empty_string_in_list() {
        assert_eq!(
            longest_common_prefix(to_vec(vec!["", "hello", "hey"])),
            ""
        );
    }

    #[test]
    fn test_single_char_prefix() {
        assert_eq!(
            longest_common_prefix(to_vec(vec!["ab", "ac", "ad"])),
            "a"
        );
    }
}
