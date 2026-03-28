pub fn next_permutation(nums: &mut Vec<i32>) {
    // Implement your solution here
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let mut nums = vec![1, 2, 3];
        next_permutation(&mut nums);
        assert_eq!(nums, vec![1, 3, 2]);
    }

    #[test]
    fn test_example_2() {
        let mut nums = vec![3, 2, 1];
        next_permutation(&mut nums);
        assert_eq!(nums, vec![1, 2, 3]);
    }

    #[test]
    fn test_example_3() {
        let mut nums = vec![1, 1, 5];
        next_permutation(&mut nums);
        assert_eq!(nums, vec![1, 5, 1]);
    }

    #[test]
    fn test_example_4() {
        let mut nums = vec![1, 3, 2];
        next_permutation(&mut nums);
        assert_eq!(nums, vec![2, 1, 3]);
    }

    #[test]
    fn test_single_element() {
        let mut nums = vec![1];
        next_permutation(&mut nums);
        assert_eq!(nums, vec![1]);
    }

    #[test]
    fn test_two_elements() {
        let mut nums = vec![1, 2];
        next_permutation(&mut nums);
        assert_eq!(nums, vec![2, 1]);
    }

    #[test]
    fn test_two_elements_descending() {
        let mut nums = vec![2, 1];
        next_permutation(&mut nums);
        assert_eq!(nums, vec![1, 2]);
    }

    #[test]
    fn test_duplicates() {
        let mut nums = vec![2, 3, 1, 3, 3];
        next_permutation(&mut nums);
        assert_eq!(nums, vec![2, 3, 3, 1, 3]);
    }

    #[test]
    fn test_all_same() {
        let mut nums = vec![1, 1, 1];
        next_permutation(&mut nums);
        assert_eq!(nums, vec![1, 1, 1]);
    }
}
