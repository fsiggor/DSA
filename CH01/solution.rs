pub fn remove_nth_from_end(list: Vec<i32>, n: i32) -> Vec<i32> {
    // Implement your solution here
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(remove_nth_from_end(vec![1, 2, 3, 4, 5], 2), vec![1, 2, 3, 5]);
    }

    #[test]
    fn test_example_2() {
        let result: Vec<i32> = vec![];
        assert_eq!(remove_nth_from_end(vec![1], 1), result);
    }

    #[test]
    fn test_example_3() {
        assert_eq!(remove_nth_from_end(vec![1, 2], 1), vec![1]);
    }

    #[test]
    fn test_remove_first_element() {
        assert_eq!(remove_nth_from_end(vec![1, 2], 2), vec![2]);
    }

    #[test]
    fn test_remove_from_three_elements() {
        assert_eq!(remove_nth_from_end(vec![1, 2, 3], 2), vec![1, 3]);
    }

    #[test]
    fn test_remove_last() {
        assert_eq!(remove_nth_from_end(vec![1, 2, 3], 1), vec![1, 2]);
    }

    #[test]
    fn test_remove_first_from_long_list() {
        assert_eq!(
            remove_nth_from_end(vec![1, 2, 3, 4, 5], 5),
            vec![2, 3, 4, 5]
        );
    }

    #[test]
    fn test_single_element_list() {
        let result: Vec<i32> = vec![];
        assert_eq!(remove_nth_from_end(vec![42], 1), result);
    }
}
