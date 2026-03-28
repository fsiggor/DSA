pub fn find_peak_element(nums: Vec<i32>) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn is_peak(nums: &[i32], idx: usize) -> bool {
        let left_ok = idx == 0 || nums[idx] > nums[idx - 1];
        let right_ok = idx == nums.len() - 1 || nums[idx] > nums[idx + 1];
        left_ok && right_ok
    }

    #[test]
    fn test_example_1() {
        let nums = vec![1, 2, 3, 1];
        let result = find_peak_element(nums.clone()) as usize;
        assert!(is_peak(&nums, result));
    }

    #[test]
    fn test_example_2() {
        let nums = vec![1, 2, 1, 3, 5, 6, 4];
        let result = find_peak_element(nums.clone()) as usize;
        assert!(is_peak(&nums, result));
    }

    #[test]
    fn test_single_element() {
        assert_eq!(find_peak_element(vec![1]), 0);
    }

    #[test]
    fn test_two_elements_ascending() {
        let nums = vec![1, 2];
        let result = find_peak_element(nums.clone()) as usize;
        assert!(is_peak(&nums, result));
    }

    #[test]
    fn test_two_elements_descending() {
        let nums = vec![2, 1];
        let result = find_peak_element(nums.clone()) as usize;
        assert!(is_peak(&nums, result));
    }

    #[test]
    fn test_ascending() {
        let nums = vec![1, 2, 3, 4, 5];
        let result = find_peak_element(nums.clone()) as usize;
        assert!(is_peak(&nums, result));
    }

    #[test]
    fn test_descending() {
        let nums = vec![5, 4, 3, 2, 1];
        let result = find_peak_element(nums.clone()) as usize;
        assert!(is_peak(&nums, result));
    }

    #[test]
    fn test_peak_in_middle() {
        let nums = vec![1, 3, 2];
        assert_eq!(find_peak_element(nums), 1);
    }

    #[test]
    fn test_multiple_peaks() {
        let nums = vec![1, 3, 1, 5, 1];
        let result = find_peak_element(nums.clone()) as usize;
        assert!(is_peak(&nums, result));
    }
}
