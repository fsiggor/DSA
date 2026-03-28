pub fn three_sum(nums: &[i32]) -> Vec<Vec<i32>> {
    // Implement your solution here
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sorted(mut result: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        for triplet in result.iter_mut() {
            triplet.sort();
        }
        result.sort();
        result
    }

    #[test]
    fn test_example_1() {
        let result = sorted(three_sum(&[-1, 0, 1, 2, -1, -4]));
        let expected = sorted(vec![vec![-1, -1, 2], vec![-1, 0, 1]]);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_example_2() {
        let result = three_sum(&[0, 1, 1]);
        assert!(result.is_empty());
    }

    #[test]
    fn test_example_3() {
        let result = sorted(three_sum(&[0, 0, 0]));
        assert_eq!(result, vec![vec![0, 0, 0]]);
    }

    #[test]
    fn test_no_triplets() {
        let result = three_sum(&[1, 2, 3]);
        assert!(result.is_empty());
    }

    #[test]
    fn test_multiple_zeros() {
        let result = sorted(three_sum(&[0, 0, 0, 0]));
        assert_eq!(result, vec![vec![0, 0, 0]]);
    }

    #[test]
    fn test_all_negative() {
        let result = three_sum(&[-1, -2, -3]);
        assert!(result.is_empty());
    }

    #[test]
    fn test_mixed() {
        let result = sorted(three_sum(&[-2, 0, 1, 1, 2]));
        let expected = sorted(vec![vec![-2, 0, 2], vec![-2, 1, 1]]);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_large_values() {
        let result = sorted(three_sum(&[-100000, 50000, 50000]));
        assert_eq!(result, vec![vec![-100000, 50000, 50000]]);
    }
}
