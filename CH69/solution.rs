pub fn length_of_lis(nums: Vec<i32>) -> i32 {
    // Implement your solution here
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(length_of_lis(vec![10, 9, 2, 5, 3, 7, 101, 18]), 4);
    }

    #[test]
    fn test_example_2() {
        assert_eq!(length_of_lis(vec![0, 1, 0, 3, 2, 3]), 4);
    }

    #[test]
    fn test_all_same() {
        assert_eq!(length_of_lis(vec![7, 7, 7, 7, 7, 7, 7]), 1);
    }

    #[test]
    fn test_single_element() {
        assert_eq!(length_of_lis(vec![10]), 1);
    }

    #[test]
    fn test_already_sorted() {
        assert_eq!(length_of_lis(vec![1, 2, 3, 4, 5]), 5);
    }

    #[test]
    fn test_reverse_sorted() {
        assert_eq!(length_of_lis(vec![5, 4, 3, 2, 1]), 1);
    }

    #[test]
    fn test_two_elements_increasing() {
        assert_eq!(length_of_lis(vec![1, 2]), 2);
    }

    #[test]
    fn test_two_elements_decreasing() {
        assert_eq!(length_of_lis(vec![2, 1]), 1);
    }

    #[test]
    fn test_negative_numbers() {
        assert_eq!(length_of_lis(vec![-5, -3, -1, 0, 2]), 5);
    }

    #[test]
    fn test_mixed() {
        assert_eq!(length_of_lis(vec![3, 1, 4, 1, 5, 9, 2, 6]), 5);
    }
}
