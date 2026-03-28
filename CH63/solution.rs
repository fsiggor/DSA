pub fn is_valid(s: String) -> bool {
    // Implement your solution here
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_parentheses() {
        assert_eq!(is_valid("()".to_string()), true);
    }

    #[test]
    fn test_multiple_types() {
        assert_eq!(is_valid("()[]{}".to_string()), true);
    }

    #[test]
    fn test_mismatched() {
        assert_eq!(is_valid("(]".to_string()), false);
    }

    #[test]
    fn test_wrong_order() {
        assert_eq!(is_valid("([)]".to_string()), false);
    }

    #[test]
    fn test_nested() {
        assert_eq!(is_valid("{[]}".to_string()), true);
    }

    #[test]
    fn test_single_open() {
        assert_eq!(is_valid("(".to_string()), false);
    }

    #[test]
    fn test_single_close() {
        assert_eq!(is_valid(")".to_string()), false);
    }

    #[test]
    fn test_complex_valid() {
        assert_eq!(is_valid("({[]})".to_string()), true);
    }
}
