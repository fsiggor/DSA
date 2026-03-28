pub fn sort_colors(nums: &mut Vec<i32>) {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let mut nums = vec![2, 0, 2, 1, 1, 0];
        sort_colors(&mut nums);
        assert_eq!(nums, vec![0, 0, 1, 1, 2, 2]);
    }

    #[test]
    fn test_example_2() {
        let mut nums = vec![2, 0, 1];
        sort_colors(&mut nums);
        assert_eq!(nums, vec![0, 1, 2]);
    }

    #[test]
    fn test_single_element() {
        let mut nums = vec![0];
        sort_colors(&mut nums);
        assert_eq!(nums, vec![0]);
    }

    #[test]
    fn test_already_sorted() {
        let mut nums = vec![0, 0, 1, 1, 2, 2];
        sort_colors(&mut nums);
        assert_eq!(nums, vec![0, 0, 1, 1, 2, 2]);
    }

    #[test]
    fn test_reverse_sorted() {
        let mut nums = vec![2, 2, 1, 1, 0, 0];
        sort_colors(&mut nums);
        assert_eq!(nums, vec![0, 0, 1, 1, 2, 2]);
    }

    #[test]
    fn test_all_same_color() {
        let mut nums = vec![1, 1, 1];
        sort_colors(&mut nums);
        assert_eq!(nums, vec![1, 1, 1]);
    }

    #[test]
    fn test_only_zeros_and_twos() {
        let mut nums = vec![2, 0, 2, 0];
        sort_colors(&mut nums);
        assert_eq!(nums, vec![0, 0, 2, 2]);
    }

    #[test]
    fn test_two_elements() {
        let mut nums = vec![1, 0];
        sort_colors(&mut nums);
        assert_eq!(nums, vec![0, 1]);
    }

    #[test]
    fn test_all_twos() {
        let mut nums = vec![2, 2, 2];
        sort_colors(&mut nums);
        assert_eq!(nums, vec![2, 2, 2]);
    }
}
