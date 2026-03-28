pub fn sort_list(list: Vec<i32>) -> Vec<i32> {
    // Implement your solution here
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(sort_list(vec![4, 2, 1, 3]), vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_example_2() {
        assert_eq!(sort_list(vec![-1, 5, 3, 4, 0]), vec![-1, 0, 3, 4, 5]);
    }

    #[test]
    fn test_empty_list() {
        assert_eq!(sort_list(vec![]), vec![]);
    }

    #[test]
    fn test_single_element() {
        assert_eq!(sort_list(vec![1]), vec![1]);
    }

    #[test]
    fn test_already_sorted() {
        assert_eq!(sort_list(vec![1, 2, 3, 4, 5]), vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_reverse_sorted() {
        assert_eq!(sort_list(vec![5, 4, 3, 2, 1]), vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_duplicates() {
        assert_eq!(sort_list(vec![3, 1, 2, 3, 1]), vec![1, 1, 2, 3, 3]);
    }

    #[test]
    fn test_all_same() {
        assert_eq!(sort_list(vec![5, 5, 5, 5]), vec![5, 5, 5, 5]);
    }

    #[test]
    fn test_negative_numbers() {
        assert_eq!(sort_list(vec![-3, -1, -2, -5, -4]), vec![-5, -4, -3, -2, -1]);
    }

    #[test]
    fn test_mixed_positive_negative() {
        assert_eq!(sort_list(vec![3, -1, 0, -2, 4]), vec![-2, -1, 0, 3, 4]);
    }
}
