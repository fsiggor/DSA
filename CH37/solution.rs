pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    // Implement your solution here
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let mut nums = vec![1, 1, 2];
        let k = remove_duplicates(&mut nums);
        assert_eq!(k, 2);
        assert_eq!(&nums[..k as usize], &[1, 2]);
    }

    #[test]
    fn test_example_2() {
        let mut nums = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
        let k = remove_duplicates(&mut nums);
        assert_eq!(k, 5);
        assert_eq!(&nums[..k as usize], &[0, 1, 2, 3, 4]);
    }

    #[test]
    fn test_single_element() {
        let mut nums = vec![1];
        let k = remove_duplicates(&mut nums);
        assert_eq!(k, 1);
        assert_eq!(&nums[..k as usize], &[1]);
    }

    #[test]
    fn test_no_duplicates() {
        let mut nums = vec![1, 2, 3, 4, 5];
        let k = remove_duplicates(&mut nums);
        assert_eq!(k, 5);
        assert_eq!(&nums[..k as usize], &[1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_all_duplicates() {
        let mut nums = vec![7, 7, 7, 7, 7];
        let k = remove_duplicates(&mut nums);
        assert_eq!(k, 1);
        assert_eq!(&nums[..k as usize], &[7]);
    }

    #[test]
    fn test_negative_numbers() {
        let mut nums = vec![-3, -3, -1, 0, 0, 2];
        let k = remove_duplicates(&mut nums);
        assert_eq!(k, 4);
        assert_eq!(&nums[..k as usize], &[-3, -1, 0, 2]);
    }

    #[test]
    fn test_two_elements_same() {
        let mut nums = vec![1, 1];
        let k = remove_duplicates(&mut nums);
        assert_eq!(k, 1);
        assert_eq!(&nums[..k as usize], &[1]);
    }

    #[test]
    fn test_two_elements_different() {
        let mut nums = vec![1, 2];
        let k = remove_duplicates(&mut nums);
        assert_eq!(k, 2);
        assert_eq!(&nums[..k as usize], &[1, 2]);
    }
}
