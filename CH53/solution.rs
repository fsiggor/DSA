/// Calculate the maximum depth of a binary tree represented as a level-order vector.
/// None values represent absent nodes.
pub fn max_depth(tree: Vec<Option<i32>>) -> i32 {
    // Implement your solution here
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let tree = vec![Some(3), Some(9), Some(20), None, None, Some(15), Some(7)];
        assert_eq!(max_depth(tree), 3);
    }

    #[test]
    fn test_example_2() {
        let tree = vec![Some(1), None, Some(2)];
        assert_eq!(max_depth(tree), 2);
    }

    #[test]
    fn test_empty() {
        assert_eq!(max_depth(vec![]), 0);
    }

    #[test]
    fn test_single_node() {
        assert_eq!(max_depth(vec![Some(1)]), 1);
    }

    #[test]
    fn test_full_tree_depth_3() {
        let tree = vec![
            Some(1),
            Some(2), Some(3),
            Some(4), Some(5), Some(6), Some(7),
        ];
        assert_eq!(max_depth(tree), 3);
    }

    #[test]
    fn test_only_left() {
        let tree = vec![Some(1), Some(2), None, Some(3)];
        assert_eq!(max_depth(tree), 3);
    }
}
