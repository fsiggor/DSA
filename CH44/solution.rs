pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_abcde_ace() {
        assert_eq!(longest_common_subsequence("abcde".to_string(), "ace".to_string()), 3);
    }

    #[test]
    fn test_identical() {
        assert_eq!(longest_common_subsequence("abc".to_string(), "abc".to_string()), 3);
    }

    #[test]
    fn test_no_common() {
        assert_eq!(longest_common_subsequence("abc".to_string(), "def".to_string()), 0);
    }

    #[test]
    fn test_single_char_match() {
        assert_eq!(longest_common_subsequence("a".to_string(), "a".to_string()), 1);
    }

    #[test]
    fn test_single_char_no_match() {
        assert_eq!(longest_common_subsequence("a".to_string(), "b".to_string()), 0);
    }

    #[test]
    fn test_one_is_subsequence() {
        assert_eq!(longest_common_subsequence("abcde".to_string(), "bd".to_string()), 2);
    }

    #[test]
    fn test_interleaved() {
        assert_eq!(longest_common_subsequence("oxcpqrsvwf".to_string(), "shmtulqrypy".to_string()), 2);
    }

    #[test]
    fn test_repeated_chars() {
        assert_eq!(longest_common_subsequence("aaa".to_string(), "aaaa".to_string()), 3);
    }
}

fn main() {}
