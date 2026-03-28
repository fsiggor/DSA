pub fn min_distance(word1: String, word2: String) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_horse_ros() {
        assert_eq!(min_distance("horse".to_string(), "ros".to_string()), 3);
    }

    #[test]
    fn test_intention_execution() {
        assert_eq!(min_distance("intention".to_string(), "execution".to_string()), 5);
    }

    #[test]
    fn test_empty_to_empty() {
        assert_eq!(min_distance("".to_string(), "".to_string()), 0);
    }

    #[test]
    fn test_empty_to_word() {
        assert_eq!(min_distance("".to_string(), "abc".to_string()), 3);
    }

    #[test]
    fn test_word_to_empty() {
        assert_eq!(min_distance("abc".to_string(), "".to_string()), 3);
    }

    #[test]
    fn test_same_word() {
        assert_eq!(min_distance("abc".to_string(), "abc".to_string()), 0);
    }

    #[test]
    fn test_single_replace() {
        assert_eq!(min_distance("a".to_string(), "b".to_string()), 1);
    }

    #[test]
    fn test_completely_different() {
        assert_eq!(min_distance("abc".to_string(), "xyz".to_string()), 3);
    }
}

fn main() {}
