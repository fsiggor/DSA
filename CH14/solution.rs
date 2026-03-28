pub fn is_valid_bst(tree: Vec<Option<i32>>) -> bool {
    // Implement your solution here
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_bst_simple() {
        assert_eq!(is_valid_bst(vec![Some(2), Some(1), Some(3)]), true);
    }

    #[test]
    fn test_invalid_bst() {
        assert_eq!(
            is_valid_bst(vec![Some(5), Some(1), Some(4), None, None, Some(3), Some(6)]),
            false
        );
    }

    #[test]
    fn test_single_node() {
        assert_eq!(is_valid_bst(vec![Some(1)]), true);
    }

    #[test]
    fn test_left_child_equal_to_root() {
        assert_eq!(is_valid_bst(vec![Some(2), Some(2), Some(3)]), false);
    }

    #[test]
    fn test_right_child_equal_to_root() {
        assert_eq!(is_valid_bst(vec![Some(2), Some(1), Some(2)]), false);
    }

    #[test]
    fn test_valid_larger_tree() {
        assert_eq!(
            is_valid_bst(vec![Some(4), Some(2), Some(6), Some(1), Some(3), Some(5), Some(7)]),
            true
        );
    }

    #[test]
    fn test_invalid_deep_violation() {
        // Root=5, left=1, right=6, right's children=3,7
        // 3 is less than 5 (root), so it violates BST in the right subtree
        assert_eq!(
            is_valid_bst(vec![Some(5), Some(1), Some(6), None, None, Some(3), Some(7)]),
            false
        );
    }

    #[test]
    fn test_left_only_tree() {
        assert_eq!(is_valid_bst(vec![Some(3), Some(2), None, Some(1)]), true);
    }

    #[test]
    fn test_negative_values() {
        assert_eq!(is_valid_bst(vec![Some(0), Some(-1), Some(1)]), true);
    }

    #[test]
    fn test_empty_tree() {
        assert_eq!(is_valid_bst(vec![]), true);
    }
}
