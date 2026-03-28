pub fn product_except_self(nums: &[i32]) -> Vec<i32> {
    // Implement your solution here
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(product_except_self(&[1, 2, 3, 4]), vec![24, 12, 8, 6]);
    }

    #[test]
    fn test_example_2() {
        assert_eq!(product_except_self(&[-1, 1, 0, -3, 3]), vec![0, 0, 9, 0, 0]);
    }

    #[test]
    fn test_example_3() {
        assert_eq!(product_except_self(&[2, 3]), vec![3, 2]);
    }

    #[test]
    fn test_contains_zero() {
        assert_eq!(product_except_self(&[0, 1, 2, 3]), vec![6, 0, 0, 0]);
    }

    #[test]
    fn test_two_zeros() {
        assert_eq!(product_except_self(&[0, 0, 2, 3]), vec![0, 0, 0, 0]);
    }

    #[test]
    fn test_all_ones() {
        assert_eq!(product_except_self(&[1, 1, 1, 1]), vec![1, 1, 1, 1]);
    }

    #[test]
    fn test_negative_numbers() {
        assert_eq!(product_except_self(&[-1, -2, -3]), vec![6, 3, 2]);
    }

    #[test]
    fn test_mixed_signs() {
        assert_eq!(product_except_self(&[-1, 2, -3, 4]), vec![-24, 12, -8, 6]);
    }
}
