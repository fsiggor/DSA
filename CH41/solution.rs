pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    // Implement your solution here
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_found() {
        assert_eq!(search(vec![-1, 0, 3, 5, 9, 12], 9), 4);
    }

    #[test]
    fn test_not_found() {
        assert_eq!(search(vec![-1, 0, 3, 5, 9, 12], 2), -1);
    }

    #[test]
    fn test_first_element() {
        assert_eq!(search(vec![1, 2, 3, 4, 5], 1), 0);
    }

    #[test]
    fn test_last_element() {
        assert_eq!(search(vec![1, 2, 3, 4, 5], 5), 4);
    }

    #[test]
    fn test_single_element_found() {
        assert_eq!(search(vec![5], 5), 0);
    }

    #[test]
    fn test_single_element_not_found() {
        assert_eq!(search(vec![5], 3), -1);
    }

    #[test]
    fn test_negative_numbers() {
        assert_eq!(search(vec![-10, -5, 0, 5, 10], -5), 1);
    }
}
