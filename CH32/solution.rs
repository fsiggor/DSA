/// Invert a binary tree represented as a level-order vector.
/// Returns a new vector with the inverted tree.
pub fn invert_tree(tree: Vec<Option<i32>>) -> Vec<Option<i32>> {
    // Implement your solution here
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_full_tree() {
        assert_eq!(
            invert_tree(vec![Some(4), Some(2), Some(7), Some(1), Some(3), Some(6), Some(9)]),
            vec![Some(4), Some(7), Some(2), Some(9), Some(6), Some(3), Some(1)]
        );
    }

    #[test]
    fn test_small_tree() {
        assert_eq!(
            invert_tree(vec![Some(2), Some(1), Some(3)]),
            vec![Some(2), Some(3), Some(1)]
        );
    }

    #[test]
    fn test_empty() {
        let empty: Vec<Option<i32>> = vec![];
        assert_eq!(invert_tree(vec![]), empty);
    }

    #[test]
    fn test_single_node() {
        assert_eq!(invert_tree(vec![Some(1)]), vec![Some(1)]);
    }

    #[test]
    fn test_with_none() {
        assert_eq!(
            invert_tree(vec![Some(1), Some(2), None]),
            vec![Some(1), None, Some(2)]
        );
    }
}
