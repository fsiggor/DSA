pub fn max_sub_array(nums: Vec<i32>) -> i32 {
    // Implement your solution here
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]), 6);
    }

    #[test]
    fn test_single_element() {
        assert_eq!(max_sub_array(vec![1]), 1);
    }

    #[test]
    fn test_all_positive() {
        assert_eq!(max_sub_array(vec![5, 4, -1, 7, 8]), 23);
    }

    #[test]
    fn test_all_negative() {
        assert_eq!(max_sub_array(vec![-3, -2, -5, -1, -4]), -1);
    }

    #[test]
    fn test_mixed() {
        assert_eq!(max_sub_array(vec![-1, 2, 3, -4, 5]), 6);
    }

    #[test]
    fn test_single_negative() {
        assert_eq!(max_sub_array(vec![-1]), -1);
    }
}
