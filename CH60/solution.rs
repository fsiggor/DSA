pub fn find_min(nums: &[i32]) -> i32 {
    // Implement your solution here
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(find_min(&[3, 4, 5, 1, 2]), 1);
    }

    #[test]
    fn test_example_2() {
        assert_eq!(find_min(&[4, 5, 6, 7, 0, 1, 2]), 0);
    }

    #[test]
    fn test_example_3() {
        assert_eq!(find_min(&[11, 13, 15, 17]), 11);
    }

    #[test]
    fn test_single_element() {
        assert_eq!(find_min(&[1]), 1);
    }

    #[test]
    fn test_two_elements_rotated() {
        assert_eq!(find_min(&[2, 1]), 1);
    }

    #[test]
    fn test_two_elements_sorted() {
        assert_eq!(find_min(&[1, 2]), 1);
    }

    #[test]
    fn test_min_at_end() {
        assert_eq!(find_min(&[2, 3, 4, 5, 1]), 1);
    }

    #[test]
    fn test_min_at_start() {
        assert_eq!(find_min(&[1, 2, 3, 4, 5]), 1);
    }

    #[test]
    fn test_negative_numbers() {
        assert_eq!(find_min(&[1, 2, -5, -3, -1]), -5);
    }
}
