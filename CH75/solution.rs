/// Merge two sorted vectors into a single sorted vector.
pub fn merge_two_lists(list1: Vec<i32>, list2: Vec<i32>) -> Vec<i32> {
    // Implement your solution here
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(
            merge_two_lists(vec![1, 2, 4], vec![1, 3, 4]),
            vec![1, 1, 2, 3, 4, 4]
        );
    }

    #[test]
    fn test_both_empty() {
        assert_eq!(merge_two_lists(vec![], vec![]), vec![]);
    }

    #[test]
    fn test_first_empty() {
        assert_eq!(merge_two_lists(vec![], vec![0]), vec![0]);
    }

    #[test]
    fn test_second_empty() {
        assert_eq!(merge_two_lists(vec![1, 2, 3], vec![]), vec![1, 2, 3]);
    }

    #[test]
    fn test_no_overlap() {
        assert_eq!(
            merge_two_lists(vec![1, 2, 3], vec![4, 5, 6]),
            vec![1, 2, 3, 4, 5, 6]
        );
    }

    #[test]
    fn test_interleaved() {
        assert_eq!(
            merge_two_lists(vec![1, 3, 5], vec![2, 4, 6]),
            vec![1, 2, 3, 4, 5, 6]
        );
    }
}
