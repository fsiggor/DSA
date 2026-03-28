pub fn add_two_numbers(l1: Vec<i32>, l2: Vec<i32>) -> Vec<i32> {
    // Implement your solution here
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(add_two_numbers(vec![2, 4, 3], vec![5, 6, 4]), vec![7, 0, 8]);
    }

    #[test]
    fn test_example_2() {
        assert_eq!(add_two_numbers(vec![0], vec![0]), vec![0]);
    }

    #[test]
    fn test_example_3() {
        assert_eq!(
            add_two_numbers(vec![9, 9, 9, 9, 9, 9, 9], vec![9, 9, 9, 9]),
            vec![8, 9, 9, 9, 0, 0, 0, 1]
        );
    }

    #[test]
    fn test_different_lengths() {
        assert_eq!(add_two_numbers(vec![1], vec![9, 9]), vec![0, 0, 1]);
    }

    #[test]
    fn test_carry_propagation() {
        assert_eq!(add_two_numbers(vec![9, 9], vec![1]), vec![0, 0, 1]);
    }

    #[test]
    fn test_no_carry() {
        assert_eq!(add_two_numbers(vec![1, 2, 3], vec![4, 5, 6]), vec![5, 7, 9]);
    }

    #[test]
    fn test_single_digits() {
        assert_eq!(add_two_numbers(vec![5], vec![5]), vec![0, 1]);
    }

    #[test]
    fn test_one_is_zero() {
        assert_eq!(add_two_numbers(vec![0], vec![1, 2, 3]), vec![1, 2, 3]);
    }

    #[test]
    fn test_large_carry_chain() {
        assert_eq!(
            add_two_numbers(vec![9, 9, 9], vec![1]),
            vec![0, 0, 0, 1]
        );
    }
}
