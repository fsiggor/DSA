pub fn right_side_view(tree: Vec<Option<i32>>) -> Vec<i32> {
    // Implement your solution here
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let tree = vec![Some(1), Some(2), Some(3), None, Some(5), None, Some(4)];
        assert_eq!(right_side_view(tree), vec![1, 3, 4]);
    }

    #[test]
    fn test_right_only() {
        let tree = vec![Some(1), None, Some(3)];
        assert_eq!(right_side_view(tree), vec![1, 3]);
    }

    #[test]
    fn test_empty_tree() {
        let tree: Vec<Option<i32>> = vec![];
        let expected: Vec<i32> = vec![];
        assert_eq!(right_side_view(tree), expected);
    }

    #[test]
    fn test_single_node() {
        let tree = vec![Some(1)];
        assert_eq!(right_side_view(tree), vec![1]);
    }

    #[test]
    fn test_left_deeper_than_right() {
        let tree = vec![Some(1), Some(2), Some(3), Some(4)];
        assert_eq!(right_side_view(tree), vec![1, 3, 4]);
    }

    #[test]
    fn test_complete_tree() {
        let tree = vec![Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7)];
        assert_eq!(right_side_view(tree), vec![1, 3, 7]);
    }

    #[test]
    fn test_left_skewed() {
        let tree = vec![Some(1), Some(2), None, Some(3)];
        assert_eq!(right_side_view(tree), vec![1, 2, 3]);
    }
}
