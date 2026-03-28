pub fn search(nums: &[i32], target: i32) -> i32 {
    // Implement your solution here
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(search(&[4, 5, 6, 7, 0, 1, 2], 0), 4);
    }

    #[test]
    fn test_example_2() {
        assert_eq!(search(&[4, 5, 6, 7, 0, 1, 2], 3), -1);
    }

    #[test]
    fn test_example_3() {
        assert_eq!(search(&[1], 0), -1);
    }

    #[test]
    fn test_example_4() {
        assert_eq!(search(&[1], 1), 0);
    }

    #[test]
    fn test_target_at_pivot() {
        assert_eq!(search(&[4, 5, 6, 7, 0, 1, 2], 4), 0);
    }

    #[test]
    fn test_target_at_end() {
        assert_eq!(search(&[4, 5, 6, 7, 0, 1, 2], 2), 6);
    }

    #[test]
    fn test_not_rotated() {
        assert_eq!(search(&[1, 2, 3, 4, 5], 3), 2);
    }

    #[test]
    fn test_not_rotated_not_found() {
        assert_eq!(search(&[1, 2, 3, 4, 5], 6), -1);
    }

    #[test]
    fn test_two_elements_found() {
        assert_eq!(search(&[3, 1], 1), 1);
    }

    #[test]
    fn test_two_elements_not_found() {
        assert_eq!(search(&[3, 1], 2), -1);
    }
}
