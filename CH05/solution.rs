pub fn single_number(nums: Vec<i32>) -> i32 {
    // Implement your solution here
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(single_number(vec![2, 2, 1]), 1);
    }

    #[test]
    fn test_example_2() {
        assert_eq!(single_number(vec![4, 1, 2, 1, 2]), 4);
    }

    #[test]
    fn test_single_element() {
        assert_eq!(single_number(vec![1]), 1);
    }

    #[test]
    fn test_negative_numbers() {
        assert_eq!(single_number(vec![-1, -1, -2]), -2);
    }

    #[test]
    fn test_larger_array() {
        assert_eq!(single_number(vec![1, 3, 1, 2, 2, 3, 99]), 99);
    }
}
