pub fn max_area(height: &[i32]) -> i32 {
    // Implement your solution here
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(max_area(&[1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);
    }

    #[test]
    fn test_example_2() {
        assert_eq!(max_area(&[1, 1]), 1);
    }

    #[test]
    fn test_example_3() {
        assert_eq!(max_area(&[4, 3, 2, 1, 4]), 16);
    }

    #[test]
    fn test_decreasing() {
        assert_eq!(max_area(&[5, 4, 3, 2, 1]), 6);
    }

    #[test]
    fn test_increasing() {
        assert_eq!(max_area(&[1, 2, 3, 4, 5]), 6);
    }

    #[test]
    fn test_equal_heights() {
        assert_eq!(max_area(&[3, 3, 3, 3]), 9);
    }

    #[test]
    fn test_tall_ends() {
        assert_eq!(max_area(&[10, 1, 1, 1, 10]), 40);
    }

    #[test]
    fn test_zero_height() {
        assert_eq!(max_area(&[0, 5]), 0);
    }
}
