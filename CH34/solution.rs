pub fn letter_combinations(digits: String) -> Vec<String> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sorted(mut v: Vec<String>) -> Vec<String> {
        v.sort();
        v
    }

    #[test]
    fn test_two_digits() {
        let result = sorted(letter_combinations("23".to_string()));
        let expected = sorted(vec![
            "ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf",
        ].into_iter().map(String::from).collect());
        assert_eq!(result, expected);
    }

    #[test]
    fn test_empty_input() {
        let result = letter_combinations("".to_string());
        let expected: Vec<String> = vec![];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_single_digit() {
        let result = sorted(letter_combinations("2".to_string()));
        let expected = sorted(vec!["a", "b", "c"]
            .into_iter().map(String::from).collect());
        assert_eq!(result, expected);
    }

    #[test]
    fn test_digit_with_four_letters() {
        let result = sorted(letter_combinations("7".to_string()));
        let expected = sorted(vec!["p", "q", "r", "s"]
            .into_iter().map(String::from).collect());
        assert_eq!(result, expected);
    }

    #[test]
    fn test_three_digits() {
        let result = letter_combinations("234".to_string());
        // 3 * 3 * 3 = 27 combinations
        assert_eq!(result.len(), 27);
    }

    #[test]
    fn test_digit_9() {
        let result = sorted(letter_combinations("9".to_string()));
        let expected = sorted(vec!["w", "x", "y", "z"]
            .into_iter().map(String::from).collect());
        assert_eq!(result, expected);
    }

    #[test]
    fn test_four_digits() {
        let result = letter_combinations("2345".to_string());
        // 3 * 3 * 3 * 3 = 81
        assert_eq!(result.len(), 81);
    }
}
