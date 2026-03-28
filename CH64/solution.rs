pub fn convert(s: String, num_rows: i32) -> String {
    // Implement your solution here
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(
            convert("PAYPALISHIRING".to_string(), 3),
            "PAHNAPLSIIGYIR"
        );
    }

    #[test]
    fn test_example_2() {
        assert_eq!(
            convert("PAYPALISHIRING".to_string(), 4),
            "PINALSIGYAHRPI"
        );
    }

    #[test]
    fn test_example_3() {
        assert_eq!(convert("A".to_string(), 1), "A");
    }

    #[test]
    fn test_single_row() {
        assert_eq!(convert("ABCDEF".to_string(), 1), "ABCDEF");
    }

    #[test]
    fn test_two_rows() {
        assert_eq!(convert("ABCDEF".to_string(), 2), "ACEBDF");
    }

    #[test]
    fn test_rows_equal_length() {
        assert_eq!(convert("ABC".to_string(), 3), "ABC");
    }

    #[test]
    fn test_rows_greater_than_length() {
        assert_eq!(convert("AB".to_string(), 5), "AB");
    }

    #[test]
    fn test_single_char() {
        assert_eq!(convert("Z".to_string(), 3), "Z");
    }

    #[test]
    fn test_empty_string() {
        assert_eq!(convert("".to_string(), 1), "");
    }
}
