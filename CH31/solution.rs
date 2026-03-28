pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    // Implement your solution here
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_has_duplicate() {
        assert_eq!(contains_duplicate(vec![1, 2, 3, 1]), true);
    }

    #[test]
    fn test_no_duplicate() {
        assert_eq!(contains_duplicate(vec![1, 2, 3, 4]), false);
    }

    #[test]
    fn test_many_duplicates() {
        assert_eq!(contains_duplicate(vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2]), true);
    }

    #[test]
    fn test_single_element() {
        assert_eq!(contains_duplicate(vec![1]), false);
    }

    #[test]
    fn test_two_same() {
        assert_eq!(contains_duplicate(vec![5, 5]), true);
    }

    #[test]
    fn test_two_different() {
        assert_eq!(contains_duplicate(vec![1, 2]), false);
    }
}
