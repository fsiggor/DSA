pub fn move_zeroes(nums: &mut Vec<i32>) {
    // Implement your solution here
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let mut nums = vec![0, 1, 0, 3, 12];
        move_zeroes(&mut nums);
        assert_eq!(nums, vec![1, 3, 12, 0, 0]);
    }

    #[test]
    fn test_single_zero() {
        let mut nums = vec![0];
        move_zeroes(&mut nums);
        assert_eq!(nums, vec![0]);
    }

    #[test]
    fn test_no_zeroes() {
        let mut nums = vec![1, 2, 3];
        move_zeroes(&mut nums);
        assert_eq!(nums, vec![1, 2, 3]);
    }

    #[test]
    fn test_all_zeroes() {
        let mut nums = vec![0, 0, 0];
        move_zeroes(&mut nums);
        assert_eq!(nums, vec![0, 0, 0]);
    }

    #[test]
    fn test_zeroes_at_end() {
        let mut nums = vec![1, 2, 0, 0];
        move_zeroes(&mut nums);
        assert_eq!(nums, vec![1, 2, 0, 0]);
    }

    #[test]
    fn test_zeroes_at_start() {
        let mut nums = vec![0, 0, 1, 2];
        move_zeroes(&mut nums);
        assert_eq!(nums, vec![1, 2, 0, 0]);
    }
}
