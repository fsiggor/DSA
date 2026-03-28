pub fn max_product(nums: Vec<i32>) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_positive_and_negative() {
        assert_eq!(max_product(vec![2, 3, -2, 4]), 6);
    }

    #[test]
    fn test_with_zero() {
        assert_eq!(max_product(vec![-2, 0, -1]), 0);
    }

    #[test]
    fn test_all_negative_even() {
        assert_eq!(max_product(vec![-2, 3, -4]), 24);
    }

    #[test]
    fn test_single_element() {
        assert_eq!(max_product(vec![5]), 5);
    }

    #[test]
    fn test_single_negative() {
        assert_eq!(max_product(vec![-3]), -3);
    }

    #[test]
    fn test_two_negatives() {
        assert_eq!(max_product(vec![-2, -3]), 6);
    }

    #[test]
    fn test_all_zeros() {
        assert_eq!(max_product(vec![0, 0, 0]), 0);
    }

    #[test]
    fn test_negative_zero_negative() {
        assert_eq!(max_product(vec![-2, 0, -3]), 0);
    }

    #[test]
    fn test_mixed() {
        assert_eq!(max_product(vec![2, -5, -2, -4, 3]), 24);
    }
}

fn main() {}
