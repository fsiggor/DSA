pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
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
    fn test_example_1() {
        let result = sorted(combination_sum(vec![2, 3, 6, 7], 7));
        let expected = sorted(vec![vec![2, 2, 3], vec![7]]);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_example_2() {
        let result = sorted(combination_sum(vec![2, 3, 5], 8));
        let expected = sorted(vec![vec![2, 2, 2, 2], vec![2, 3, 3], vec![3, 5]]);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_no_combination() {
        let result = combination_sum(vec![2], 1);
        let expected: Vec<Vec<i32>> = vec![];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_single_candidate_exact() {
        let result = sorted(combination_sum(vec![3], 9));
        let expected = sorted(vec![vec![3, 3, 3]]);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_single_candidate_no_match() {
        let result = combination_sum(vec![3], 7);
        let expected: Vec<Vec<i32>> = vec![];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_target_equals_candidate() {
        let result = sorted(combination_sum(vec![2, 3, 5], 5));
        let expected = sorted(vec![vec![2, 3], vec![5]]);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_large_target() {
        let result = combination_sum(vec![2, 3, 5], 12);
        assert!(!result.is_empty());
        for combo in &result {
            assert_eq!(combo.iter().sum::<i32>(), 12);
        }
    }

    #[test]
    fn test_all_sums_correct() {
        let result = combination_sum(vec![2, 3, 6, 7], 7);
        for combo in &result {
            assert_eq!(combo.iter().sum::<i32>(), 7);
        }
    }
}
