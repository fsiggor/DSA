pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sorted(mut v: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        for s in v.iter_mut() {
            s.sort();
        }
        v.sort();
        v
    }

    #[test]
    fn test_three_elements() {
        let result = sorted(subsets(vec![1, 2, 3]));
        let expected = sorted(vec![
            vec![],
            vec![1],
            vec![2],
            vec![1, 2],
            vec![3],
            vec![1, 3],
            vec![2, 3],
            vec![1, 2, 3],
        ]);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_single_element() {
        let result = sorted(subsets(vec![0]));
        let expected = sorted(vec![vec![], vec![0]]);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_two_elements() {
        let result = sorted(subsets(vec![1, 2]));
        let expected = sorted(vec![vec![], vec![1], vec![2], vec![1, 2]]);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_power_set_size() {
        let result = subsets(vec![1, 2, 3, 4]);
        assert_eq!(result.len(), 16); // 2^4 = 16
    }

    #[test]
    fn test_contains_empty_set() {
        let result = subsets(vec![1, 2, 3]);
        assert!(result.contains(&vec![]));
    }

    #[test]
    fn test_contains_full_set() {
        let result = sorted(subsets(vec![1, 2, 3]));
        assert!(result.contains(&vec![1, 2, 3]));
    }

    #[test]
    fn test_no_duplicates() {
        let result = sorted(subsets(vec![1, 2, 3]));
        let mut unique = result.clone();
        unique.dedup();
        assert_eq!(result.len(), unique.len());
    }

    #[test]
    fn test_negative_numbers() {
        let result = sorted(subsets(vec![-1, 0]));
        let expected = sorted(vec![vec![], vec![-1], vec![0], vec![-1, 0]]);
        assert_eq!(result, expected);
    }
}
