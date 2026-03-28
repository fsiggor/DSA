pub fn level_order(tree: Vec<Option<i32>>) -> Vec<Vec<i32>> {
    // Implement your solution here
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let tree = vec![Some(3), Some(9), Some(20), None, None, Some(15), Some(7)];
        assert_eq!(level_order(tree), vec![vec![3], vec![9, 20], vec![15, 7]]);
    }

    #[test]
    fn test_single_node() {
        let tree = vec![Some(1)];
        assert_eq!(level_order(tree), vec![vec![1]]);
    }

    #[test]
    fn test_empty_tree() {
        let tree: Vec<Option<i32>> = vec![];
        let expected: Vec<Vec<i32>> = vec![];
        assert_eq!(level_order(tree), expected);
    }

    #[test]
    fn test_complete_tree() {
        let tree = vec![Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7)];
        assert_eq!(
            level_order(tree),
            vec![vec![1], vec![2, 3], vec![4, 5, 6, 7]]
        );
    }

    #[test]
    fn test_left_skewed() {
        let tree = vec![Some(1), Some(2), None, Some(3), None];
        assert_eq!(level_order(tree), vec![vec![1], vec![2], vec![3]]);
    }

    #[test]
    fn test_right_skewed() {
        let tree = vec![Some(1), None, Some(2), None, Some(3)];
        assert_eq!(level_order(tree), vec![vec![1], vec![2], vec![3]]);
    }

    #[test]
    fn test_negative_values() {
        let tree = vec![Some(-10), Some(9), Some(20), None, None, Some(-15), Some(7)];
        assert_eq!(
            level_order(tree),
            vec![vec![-10], vec![9, 20], vec![-15, 7]]
        );
    }
}
