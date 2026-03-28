pub fn first_missing_positive(nums: &mut Vec<i32>) -> i32 {
    // Implement your solution here
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let mut nums = vec![1, 2, 0];
        assert_eq!(first_missing_positive(&mut nums), 3);
    }

    #[test]
    fn test_example_2() {
        let mut nums = vec![3, 4, -1, 1];
        assert_eq!(first_missing_positive(&mut nums), 2);
    }

    #[test]
    fn test_example_3() {
        let mut nums = vec![7, 8, 9, 11, 12];
        assert_eq!(first_missing_positive(&mut nums), 1);
    }

    #[test]
    fn test_example_4() {
        let mut nums = vec![1];
        assert_eq!(first_missing_positive(&mut nums), 2);
    }

    #[test]
    fn test_consecutive_from_one() {
        let mut nums = vec![1, 2, 3, 4, 5];
        assert_eq!(first_missing_positive(&mut nums), 6);
    }

    #[test]
    fn test_all_negative() {
        let mut nums = vec![-1, -2, -3];
        assert_eq!(first_missing_positive(&mut nums), 1);
    }

    #[test]
    fn test_with_duplicates() {
        let mut nums = vec![1, 1, 1];
        assert_eq!(first_missing_positive(&mut nums), 2);
    }

    #[test]
    fn test_single_zero() {
        let mut nums = vec![0];
        assert_eq!(first_missing_positive(&mut nums), 1);
    }

    #[test]
    fn test_gap_in_middle() {
        let mut nums = vec![1, 2, 4, 5];
        assert_eq!(first_missing_positive(&mut nums), 3);
    }

    #[test]
    fn test_large_numbers() {
        let mut nums = vec![1000000, 2, 1];
        assert_eq!(first_missing_positive(&mut nums), 3);
    }
}
