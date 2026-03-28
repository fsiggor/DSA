pub fn get_intersection(list1: Vec<i32>, list2: Vec<i32>) -> i32 {
    // Implement your solution here
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intersection_exists_1() {
        assert_eq!(get_intersection(vec![4, 1, 8, 4, 5], vec![5, 6, 1, 8, 4, 5]), 1);
    }

    #[test]
    fn test_intersection_exists_2() {
        assert_eq!(get_intersection(vec![1, 9, 1, 2, 4], vec![3, 2, 4]), 2);
    }

    #[test]
    fn test_no_intersection() {
        assert_eq!(get_intersection(vec![2, 6, 4], vec![1, 5]), -1);
    }

    #[test]
    fn test_empty_first_list() {
        assert_eq!(get_intersection(vec![], vec![1, 2, 3]), -1);
    }

    #[test]
    fn test_empty_second_list() {
        assert_eq!(get_intersection(vec![1, 2, 3], vec![]), -1);
    }

    #[test]
    fn test_both_empty() {
        assert_eq!(get_intersection(vec![], vec![]), -1);
    }

    #[test]
    fn test_single_element_match() {
        assert_eq!(get_intersection(vec![5], vec![5]), 5);
    }

    #[test]
    fn test_first_element_matches() {
        assert_eq!(get_intersection(vec![3, 4, 5], vec![3, 6, 7]), 3);
    }

    #[test]
    fn test_last_element_matches() {
        assert_eq!(get_intersection(vec![1, 2, 3], vec![3]), 3);
    }
}
