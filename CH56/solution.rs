pub fn reverse_list(list: Vec<i32>) -> Vec<i32> {
    // Implement your solution here
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(reverse_list(vec![1, 2, 3, 4, 5]), vec![5, 4, 3, 2, 1]);
    }

    #[test]
    fn test_example_2() {
        assert_eq!(reverse_list(vec![1, 2]), vec![2, 1]);
    }

    #[test]
    fn test_empty() {
        assert_eq!(reverse_list(vec![]), vec![]);
    }

    #[test]
    fn test_single() {
        assert_eq!(reverse_list(vec![42]), vec![42]);
    }

    #[test]
    fn test_negative_numbers() {
        assert_eq!(reverse_list(vec![-1, -2, -3]), vec![-3, -2, -1]);
    }
}
