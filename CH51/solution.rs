pub fn rob(nums: Vec<i32>) -> i32 {
    // Implement your solution here
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(rob(vec![1, 2, 3, 1]), 4);
    }

    #[test]
    fn test_example_2() {
        assert_eq!(rob(vec![2, 7, 9, 3, 1]), 12);
    }

    #[test]
    fn test_single_house() {
        assert_eq!(rob(vec![5]), 5);
    }

    #[test]
    fn test_two_houses() {
        assert_eq!(rob(vec![1, 2]), 2);
    }

    #[test]
    fn test_all_same() {
        assert_eq!(rob(vec![3, 3, 3, 3]), 6);
    }

    #[test]
    fn test_increasing() {
        assert_eq!(rob(vec![1, 2, 3, 4, 5]), 9);
    }

    #[test]
    fn test_decreasing() {
        assert_eq!(rob(vec![5, 4, 3, 2, 1]), 9);
    }

    #[test]
    fn test_zeros() {
        assert_eq!(rob(vec![0, 0, 0, 0]), 0);
    }

    #[test]
    fn test_large_values() {
        assert_eq!(rob(vec![400, 1, 1, 400]), 800);
    }

    #[test]
    fn test_three_houses() {
        assert_eq!(rob(vec![2, 1, 1]), 3);
    }
}
