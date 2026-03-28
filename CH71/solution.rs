pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
    // Implement your solution here
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let mut result = top_k_frequent(vec![1, 1, 1, 2, 2, 3], 2);
        result.sort();
        assert_eq!(result, vec![1, 2]);
    }

    #[test]
    fn test_single_element() {
        assert_eq!(top_k_frequent(vec![1], 1), vec![1]);
    }

    #[test]
    fn test_all_same() {
        assert_eq!(top_k_frequent(vec![5, 5, 5, 5], 1), vec![5]);
    }

    #[test]
    fn test_k_equals_unique() {
        let mut result = top_k_frequent(vec![1, 2, 3], 3);
        result.sort();
        assert_eq!(result, vec![1, 2, 3]);
    }

    #[test]
    fn test_negative_numbers() {
        let mut result = top_k_frequent(vec![-1, -1, 2, 2, 2, 3], 2);
        result.sort();
        assert_eq!(result, vec![-1, 2]);
    }

    #[test]
    fn test_distinct_frequencies() {
        let mut result = top_k_frequent(vec![1, 1, 1, 2, 2, 3, 3, 3, 3], 2);
        result.sort();
        assert_eq!(result, vec![1, 3]);
    }
}
