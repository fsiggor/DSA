pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(find_kth_largest(vec![3, 2, 1, 5, 6, 4], 2), 5);
    }

    #[test]
    fn test_example_2() {
        assert_eq!(find_kth_largest(vec![3, 2, 3, 1, 2, 4, 5, 5, 6], 4), 4);
    }

    #[test]
    fn test_single_element() {
        assert_eq!(find_kth_largest(vec![1], 1), 1);
    }

    #[test]
    fn test_k_equals_length() {
        assert_eq!(find_kth_largest(vec![3, 1, 2], 3), 1);
    }

    #[test]
    fn test_duplicates() {
        assert_eq!(find_kth_largest(vec![2, 2, 2, 2], 2), 2);
    }

    #[test]
    fn test_negative_numbers() {
        assert_eq!(find_kth_largest(vec![-1, -2, -3, -4], 1), -1);
    }

    #[test]
    fn test_mixed_positive_negative() {
        assert_eq!(find_kth_largest(vec![-1, 2, 0, 3, -2], 2), 2);
    }

    #[test]
    fn test_sorted_ascending() {
        assert_eq!(find_kth_largest(vec![1, 2, 3, 4, 5], 1), 5);
    }

    #[test]
    fn test_sorted_descending() {
        assert_eq!(find_kth_largest(vec![5, 4, 3, 2, 1], 3), 3);
    }
}
