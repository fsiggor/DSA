pub fn eval_rpn(tokens: Vec<String>) -> i32 {
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
        assert_eq!(eval_rpn(to_string_vec(vec!["2", "1", "+", "3", "*"])), 9);
    }

    #[test]
    fn test_example_2() {
        assert_eq!(eval_rpn(to_string_vec(vec!["4", "13", "5", "/", "+"])), 6);
    }

    #[test]
    fn test_example_3() {
        assert_eq!(
            eval_rpn(to_string_vec(vec![
                "10", "6", "9", "3", "+", "-11", "*", "/", "*", "17", "+", "5", "+"
            ])),
            22
        );
    }

    #[test]
    fn test_single_number() {
        assert_eq!(eval_rpn(to_string_vec(vec!["42"])), 42);
    }

    #[test]
    fn test_simple_addition() {
        assert_eq!(eval_rpn(to_string_vec(vec!["1", "2", "+"])), 3);
    }

    #[test]
    fn test_simple_subtraction() {
        assert_eq!(eval_rpn(to_string_vec(vec!["5", "3", "-"])), 2);
    }

    #[test]
    fn test_simple_multiplication() {
        assert_eq!(eval_rpn(to_string_vec(vec!["3", "4", "*"])), 12);
    }

    #[test]
    fn test_simple_division() {
        assert_eq!(eval_rpn(to_string_vec(vec!["10", "3", "/"])), 3);
    }

    #[test]
    fn test_negative_numbers() {
        assert_eq!(eval_rpn(to_string_vec(vec!["-1", "-2", "+"])), -3);
    }

    #[test]
    fn test_division_truncates_toward_zero() {
        assert_eq!(eval_rpn(to_string_vec(vec!["7", "-2", "/"])), -3);
    }
}
