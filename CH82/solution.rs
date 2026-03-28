pub fn has_cycle(nums: Vec<i32>, cycle_pos: i32) -> bool {
    // Implement your solution here
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cycle_at_index_1() {
        assert_eq!(has_cycle(vec![3, 2, 0, -4], 1), true);
    }

    #[test]
    fn test_cycle_at_index_0() {
        assert_eq!(has_cycle(vec![1, 2], 0), true);
    }

    #[test]
    fn test_no_cycle() {
        assert_eq!(has_cycle(vec![1], -1), false);
    }

    #[test]
    fn test_empty_list() {
        assert_eq!(has_cycle(vec![], -1), false);
    }

    #[test]
    fn test_single_element_with_cycle() {
        assert_eq!(has_cycle(vec![1], 0), true);
    }

    #[test]
    fn test_long_list_no_cycle() {
        assert_eq!(has_cycle(vec![1, 2, 3, 4, 5], -1), false);
    }

    #[test]
    fn test_cycle_at_last_index() {
        assert_eq!(has_cycle(vec![1, 2, 3, 4], 3), true);
    }

    #[test]
    fn test_negative_values_with_cycle() {
        assert_eq!(has_cycle(vec![-1, -2, -3], 2), true);
    }
}
