pub fn trap(height: &[i32]) -> i32 {
    // Implement your solution here
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(trap(&[0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]), 6);
    }

    #[test]
    fn test_example_2() {
        assert_eq!(trap(&[4, 2, 0, 3, 2, 5]), 9);
    }

    #[test]
    fn test_example_3() {
        assert_eq!(trap(&[1, 0, 1]), 1);
    }

    #[test]
    fn test_empty() {
        assert_eq!(trap(&[]), 0);
    }

    #[test]
    fn test_single_element() {
        assert_eq!(trap(&[5]), 0);
    }

    #[test]
    fn test_two_elements() {
        assert_eq!(trap(&[3, 1]), 0);
    }

    #[test]
    fn test_no_trap_ascending() {
        assert_eq!(trap(&[1, 2, 3, 4, 5]), 0);
    }

    #[test]
    fn test_no_trap_descending() {
        assert_eq!(trap(&[5, 4, 3, 2, 1]), 0);
    }

    #[test]
    fn test_valley() {
        assert_eq!(trap(&[5, 0, 5]), 5);
    }

    #[test]
    fn test_all_zeros() {
        assert_eq!(trap(&[0, 0, 0, 0]), 0);
    }

    #[test]
    fn test_multiple_valleys() {
        assert_eq!(trap(&[3, 0, 3, 0, 3]), 6);
    }
}
