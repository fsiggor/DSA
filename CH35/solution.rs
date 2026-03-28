pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    // Implement your solution here
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let result = two_sum(vec![2, 7, 11, 15], 9);
        assert_eq!(result.len(), 2);
        let mut sorted = result.clone();
        sorted.sort();
        assert_eq!(sorted, vec![0, 1]);
    }

    #[test]
    fn test_example_2() {
        let result = two_sum(vec![3, 2, 4], 6);
        assert_eq!(result.len(), 2);
        let mut sorted = result.clone();
        sorted.sort();
        assert_eq!(sorted, vec![1, 2]);
    }

    #[test]
    fn test_example_3() {
        let result = two_sum(vec![3, 3], 6);
        assert_eq!(result.len(), 2);
        let mut sorted = result.clone();
        sorted.sort();
        assert_eq!(sorted, vec![0, 1]);
    }

    #[test]
    fn test_negative_numbers() {
        let result = two_sum(vec![-1, -2, -3, -4, -5], -8);
        assert_eq!(result.len(), 2);
        let mut sorted = result.clone();
        sorted.sort();
        assert_eq!(sorted, vec![2, 4]);
    }

    #[test]
    fn test_single_pair() {
        let result = two_sum(vec![1, 2], 3);
        assert_eq!(result.len(), 2);
        let mut sorted = result.clone();
        sorted.sort();
        assert_eq!(sorted, vec![0, 1]);
    }
}
