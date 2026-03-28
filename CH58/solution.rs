pub fn kth_smallest(tree: Vec<Option<i32>>, k: i32) -> i32 {
    // Implement your solution here
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let tree = vec![Some(3), Some(1), Some(4), None, Some(2)];
        assert_eq!(kth_smallest(tree, 1), 1);
    }

    #[test]
    fn test_example_2() {
        let tree = vec![
            Some(5), Some(3), Some(6), Some(2), Some(4), None, None, Some(1),
        ];
        assert_eq!(kth_smallest(tree, 3), 3);
    }

    #[test]
    fn test_single_node() {
        let tree = vec![Some(1)];
        assert_eq!(kth_smallest(tree, 1), 1);
    }

    #[test]
    fn test_k_equals_n() {
        let tree = vec![Some(3), Some(1), Some(4), None, Some(2)];
        assert_eq!(kth_smallest(tree, 4), 4);
    }

    #[test]
    fn test_left_skewed() {
        let tree = vec![Some(3), Some(2), None, Some(1)];
        assert_eq!(kth_smallest(tree, 2), 2);
    }

    #[test]
    fn test_right_skewed() {
        let tree = vec![Some(1), None, Some(2), None, Some(3)];
        assert_eq!(kth_smallest(tree, 3), 3);
    }

    #[test]
    fn test_k_is_root() {
        let tree = vec![Some(2), Some(1), Some(3)];
        assert_eq!(kth_smallest(tree, 2), 2);
    }
}
