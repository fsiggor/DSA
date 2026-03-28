pub fn is_symmetric(tree: Vec<Option<i32>>) -> bool {
    // Implement your solution here
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_symmetric_tree() {
        assert_eq!(
            is_symmetric(vec![Some(1), Some(2), Some(2), Some(3), Some(4), Some(4), Some(3)]),
            true
        );
    }

    #[test]
    fn test_asymmetric_tree() {
        assert_eq!(
            is_symmetric(vec![Some(1), Some(2), Some(2), None, Some(3), None, Some(3)]),
            false
        );
    }

    #[test]
    fn test_single_node() {
        assert_eq!(is_symmetric(vec![Some(1)]), true);
    }

    #[test]
    fn test_two_levels_symmetric() {
        assert_eq!(is_symmetric(vec![Some(1), Some(2), Some(2)]), true);
    }

    #[test]
    fn test_two_levels_asymmetric() {
        assert_eq!(is_symmetric(vec![Some(1), Some(2), Some(3)]), false);
    }

    #[test]
    fn test_none_children_symmetric() {
        assert_eq!(
            is_symmetric(vec![Some(1), None, None]),
            true
        );
    }

    #[test]
    fn test_one_child_none() {
        assert_eq!(
            is_symmetric(vec![Some(1), Some(2), None]),
            false
        );
    }

    #[test]
    fn test_same_values_but_different_structure() {
        assert_eq!(
            is_symmetric(vec![Some(1), Some(2), Some(2), Some(2), None, None, Some(2)]),
            true
        );
    }

    #[test]
    fn test_negative_values_symmetric() {
        assert_eq!(
            is_symmetric(vec![Some(0), Some(-1), Some(-1)]),
            true
        );
    }

    #[test]
    fn test_empty_tree() {
        assert_eq!(is_symmetric(vec![]), true);
    }
}
