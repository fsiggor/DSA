pub fn can_jump(nums: Vec<i32>) -> bool {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reachable() {
        assert_eq!(can_jump(vec![2, 3, 1, 1, 4]), true);
    }

    #[test]
    fn test_unreachable() {
        assert_eq!(can_jump(vec![3, 2, 1, 0, 4]), false);
    }

    #[test]
    fn test_single_element() {
        assert_eq!(can_jump(vec![0]), true);
    }

    #[test]
    fn test_two_elements_reachable() {
        assert_eq!(can_jump(vec![1, 0]), true);
    }

    #[test]
    fn test_two_elements_unreachable() {
        assert_eq!(can_jump(vec![0, 1]), false);
    }

    #[test]
    fn test_large_first_jump() {
        assert_eq!(can_jump(vec![10, 0, 0, 0, 0]), true);
    }

    #[test]
    fn test_all_ones() {
        assert_eq!(can_jump(vec![1, 1, 1, 1, 1]), true);
    }

    #[test]
    fn test_all_zeros_except_first() {
        assert_eq!(can_jump(vec![0, 0, 0]), false);
    }

    #[test]
    fn test_decreasing() {
        assert_eq!(can_jump(vec![4, 3, 2, 1, 0]), true);
    }
}

fn main() {}
