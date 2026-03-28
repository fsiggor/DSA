pub fn max_profit(prices: Vec<i32>) -> i32 {
    // Implement your solution here
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(max_profit(vec![7, 1, 5, 3, 6, 4]), 5);
    }

    #[test]
    fn test_example_2() {
        assert_eq!(max_profit(vec![7, 6, 4, 3, 1]), 0);
    }

    #[test]
    fn test_single_element() {
        assert_eq!(max_profit(vec![5]), 0);
    }

    #[test]
    fn test_two_elements_profit() {
        assert_eq!(max_profit(vec![1, 5]), 4);
    }

    #[test]
    fn test_two_elements_no_profit() {
        assert_eq!(max_profit(vec![5, 1]), 0);
    }

    #[test]
    fn test_increasing() {
        assert_eq!(max_profit(vec![1, 2, 3, 4, 5]), 4);
    }

    #[test]
    fn test_valley_then_peak() {
        assert_eq!(max_profit(vec![3, 1, 4, 8, 2, 9]), 8);
    }
}
