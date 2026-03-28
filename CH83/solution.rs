pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
    // Implement your solution here
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(largest_rectangle_area(vec![2, 1, 5, 6, 2, 3]), 10);
    }

    #[test]
    fn test_example_2() {
        assert_eq!(largest_rectangle_area(vec![2, 4]), 4);
    }

    #[test]
    fn test_single_bar() {
        assert_eq!(largest_rectangle_area(vec![5]), 5);
    }

    #[test]
    fn test_equal_heights() {
        assert_eq!(largest_rectangle_area(vec![3, 3, 3, 3]), 12);
    }

    #[test]
    fn test_ascending() {
        assert_eq!(largest_rectangle_area(vec![1, 2, 3, 4, 5]), 9);
    }

    #[test]
    fn test_descending() {
        assert_eq!(largest_rectangle_area(vec![5, 4, 3, 2, 1]), 9);
    }

    #[test]
    fn test_valley() {
        assert_eq!(largest_rectangle_area(vec![5, 1, 5]), 5);
    }

    #[test]
    fn test_with_zero() {
        assert_eq!(largest_rectangle_area(vec![2, 0, 2]), 2);
    }

    #[test]
    fn test_all_zeros() {
        assert_eq!(largest_rectangle_area(vec![0, 0, 0]), 0);
    }

    #[test]
    fn test_large_single_bar() {
        assert_eq!(largest_rectangle_area(vec![10000]), 10000);
    }

    #[test]
    fn test_pyramid() {
        assert_eq!(largest_rectangle_area(vec![1, 3, 5, 3, 1]), 9);
    }
}
