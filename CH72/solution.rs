pub fn lowest_common_ancestor(tree: Vec<Option<i32>>, p: i32, q: i32) -> i32 {
    // Implement your solution here
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let tree = vec![
            Some(6), Some(2), Some(8), Some(0), Some(4),
            Some(7), Some(9), None, None, Some(3), Some(5),
        ];
        assert_eq!(lowest_common_ancestor(tree, 2, 8), 6);
    }

    #[test]
    fn test_example_2() {
        let tree = vec![
            Some(6), Some(2), Some(8), Some(0), Some(4),
            Some(7), Some(9), None, None, Some(3), Some(5),
        ];
        assert_eq!(lowest_common_ancestor(tree, 2, 4), 2);
    }

    #[test]
    fn test_root_is_lca() {
        let tree = vec![Some(2), Some(1), Some(3)];
        assert_eq!(lowest_common_ancestor(tree, 1, 3), 2);
    }

    #[test]
    fn test_same_subtree() {
        let tree = vec![
            Some(6), Some(2), Some(8), Some(0), Some(4),
            Some(7), Some(9), None, None, Some(3), Some(5),
        ];
        assert_eq!(lowest_common_ancestor(tree, 3, 5), 4);
    }

    #[test]
    fn test_node_is_own_ancestor() {
        let tree = vec![
            Some(6), Some(2), Some(8), Some(0), Some(4),
            Some(7), Some(9), None, None, Some(3), Some(5),
        ];
        assert_eq!(lowest_common_ancestor(tree, 0, 4), 2);
    }

    #[test]
    fn test_two_nodes() {
        let tree = vec![Some(2), Some(1)];
        assert_eq!(lowest_common_ancestor(tree, 2, 1), 2);
    }

    #[test]
    fn test_right_subtree() {
        let tree = vec![
            Some(6), Some(2), Some(8), Some(0), Some(4),
            Some(7), Some(9), None, None, Some(3), Some(5),
        ];
        assert_eq!(lowest_common_ancestor(tree, 7, 9), 8);
    }
}
