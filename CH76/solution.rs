pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
    // Implement your solution here
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_chain() {
        assert_eq!(can_finish(2, vec![vec![1, 0]]), true);
    }

    #[test]
    fn test_cycle() {
        assert_eq!(can_finish(2, vec![vec![1, 0], vec![0, 1]]), false);
    }

    #[test]
    fn test_no_prerequisites() {
        assert_eq!(can_finish(3, vec![]), true);
    }

    #[test]
    fn test_single_course() {
        assert_eq!(can_finish(1, vec![]), true);
    }

    #[test]
    fn test_linear_chain() {
        assert_eq!(can_finish(4, vec![vec![1, 0], vec![2, 1], vec![3, 2]]), true);
    }

    #[test]
    fn test_longer_cycle() {
        assert_eq!(
            can_finish(3, vec![vec![1, 0], vec![2, 1], vec![0, 2]]),
            false
        );
    }

    #[test]
    fn test_multiple_independent() {
        assert_eq!(can_finish(4, vec![vec![1, 0], vec![3, 2]]), true);
    }

    #[test]
    fn test_diamond_shape() {
        assert_eq!(
            can_finish(4, vec![vec![1, 0], vec![2, 0], vec![3, 1], vec![3, 2]]),
            true
        );
    }
}
