pub fn merge(nums1: &mut Vec<i32>, m: usize, nums2: &Vec<i32>, n: usize) {
    // Implement your solution here
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let mut nums1 = vec![1, 2, 3, 0, 0, 0];
        let nums2 = vec![2, 5, 6];
        merge(&mut nums1, 3, &nums2, 3);
        assert_eq!(nums1, vec![1, 2, 2, 3, 5, 6]);
    }

    #[test]
    fn test_example_2() {
        let mut nums1 = vec![1];
        let nums2 = vec![];
        merge(&mut nums1, 1, &nums2, 0);
        assert_eq!(nums1, vec![1]);
    }

    #[test]
    fn test_example_3() {
        let mut nums1 = vec![0];
        let nums2 = vec![1];
        merge(&mut nums1, 0, &nums2, 1);
        assert_eq!(nums1, vec![1]);
    }

    #[test]
    fn test_nums2_all_smaller() {
        let mut nums1 = vec![4, 5, 6, 0, 0, 0];
        let nums2 = vec![1, 2, 3];
        merge(&mut nums1, 3, &nums2, 3);
        assert_eq!(nums1, vec![1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn test_nums2_all_larger() {
        let mut nums1 = vec![1, 2, 3, 0, 0, 0];
        let nums2 = vec![4, 5, 6];
        merge(&mut nums1, 3, &nums2, 3);
        assert_eq!(nums1, vec![1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn test_interleaved() {
        let mut nums1 = vec![1, 3, 5, 0, 0, 0];
        let nums2 = vec![2, 4, 6];
        merge(&mut nums1, 3, &nums2, 3);
        assert_eq!(nums1, vec![1, 2, 3, 4, 5, 6]);
    }
}
