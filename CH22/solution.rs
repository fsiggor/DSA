pub fn missing_number(nums: Vec<i32>) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(missing_number(vec![3, 0, 1]), 2);
    }

    #[test]
    fn test_example_2() {
        assert_eq!(missing_number(vec![0, 1]), 2);
    }

    #[test]
    fn test_example_3() {
        assert_eq!(missing_number(vec![9, 6, 4, 2, 3, 5, 7, 0, 1]), 8);
    }

    #[test]
    fn test_missing_zero() {
        assert_eq!(missing_number(vec![1]), 0);
    }

    #[test]
    fn test_missing_last() {
        assert_eq!(missing_number(vec![0, 1, 2]), 3);
    }

    #[test]
    fn test_single_zero() {
        assert_eq!(missing_number(vec![0]), 1);
    }

    #[test]
    fn test_larger_array() {
        assert_eq!(missing_number(vec![0, 1, 2, 3, 4, 6, 7, 8, 9]), 5);
    }

    #[test]
    fn test_missing_middle() {
        assert_eq!(missing_number(vec![0, 2, 3, 4, 5]), 1);
    }

    #[test]
    fn test_two_elements_missing_one() {
        assert_eq!(missing_number(vec![0, 2]), 1);
    }
}
