pub fn rotate(nums: &mut Vec<i32>, k: usize) {
    // Implement your solution here
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let mut nums = vec![1, 2, 3, 4, 5, 6, 7];
        rotate(&mut nums, 3);
        assert_eq!(nums, vec![5, 6, 7, 1, 2, 3, 4]);
    }

    #[test]
    fn test_example_2() {
        let mut nums = vec![-1, -100, 3, 99];
        rotate(&mut nums, 2);
        assert_eq!(nums, vec![3, 99, -1, -100]);
    }

    #[test]
    fn test_example_3() {
        let mut nums = vec![1, 2];
        rotate(&mut nums, 3);
        assert_eq!(nums, vec![2, 1]);
    }

    #[test]
    fn test_k_zero() {
        let mut nums = vec![1, 2, 3];
        rotate(&mut nums, 0);
        assert_eq!(nums, vec![1, 2, 3]);
    }

    #[test]
    fn test_k_equals_len() {
        let mut nums = vec![1, 2, 3];
        rotate(&mut nums, 3);
        assert_eq!(nums, vec![1, 2, 3]);
    }

    #[test]
    fn test_k_greater_than_len() {
        let mut nums = vec![1, 2, 3, 4, 5];
        rotate(&mut nums, 7);
        assert_eq!(nums, vec![4, 5, 1, 2, 3]);
    }

    #[test]
    fn test_single_element() {
        let mut nums = vec![1];
        rotate(&mut nums, 5);
        assert_eq!(nums, vec![1]);
    }

    #[test]
    fn test_rotate_by_one() {
        let mut nums = vec![1, 2, 3, 4];
        rotate(&mut nums, 1);
        assert_eq!(nums, vec![4, 1, 2, 3]);
    }
}
