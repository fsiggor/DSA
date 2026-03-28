pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Vec<Option<i32>> {
    // Implement your solution here
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let preorder = vec![3, 9, 20, 15, 7];
        let inorder = vec![9, 3, 15, 20, 7];
        assert_eq!(
            build_tree(preorder, inorder),
            vec![Some(3), Some(9), Some(20), None, None, Some(15), Some(7)]
        );
    }

    #[test]
    fn test_single_node() {
        let preorder = vec![-1];
        let inorder = vec![-1];
        assert_eq!(build_tree(preorder, inorder), vec![Some(-1)]);
    }

    #[test]
    fn test_left_only() {
        let preorder = vec![1, 2];
        let inorder = vec![2, 1];
        assert_eq!(
            build_tree(preorder, inorder),
            vec![Some(1), Some(2)]
        );
    }

    #[test]
    fn test_right_only() {
        let preorder = vec![1, 2];
        let inorder = vec![1, 2];
        assert_eq!(
            build_tree(preorder, inorder),
            vec![Some(1), None, Some(2)]
        );
    }

    #[test]
    fn test_complete_tree() {
        let preorder = vec![1, 2, 4, 5, 3, 6, 7];
        let inorder = vec![4, 2, 5, 1, 6, 3, 7];
        assert_eq!(
            build_tree(preorder, inorder),
            vec![Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7)]
        );
    }

    #[test]
    fn test_negative_values() {
        let preorder = vec![-10, -20, -30];
        let inorder = vec![-30, -20, -10];
        assert_eq!(
            build_tree(preorder, inorder),
            vec![Some(-10), Some(-20), None, Some(-30)]
        );
    }
}
