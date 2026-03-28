pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sorted(mut v: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        v.sort();
        v
    }

    #[test]
    fn test_three_elements() {
        let result = sorted(permute(vec![1, 2, 3]));
        let expected = sorted(vec![
            vec![1, 2, 3],
            vec![1, 3, 2],
            vec![2, 1, 3],
            vec![2, 3, 1],
            vec![3, 1, 2],
            vec![3, 2, 1],
        ]);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_two_elements() {
        let result = sorted(permute(vec![0, 1]));
        let expected = sorted(vec![vec![0, 1], vec![1, 0]]);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_single_element() {
        assert_eq!(permute(vec![1]), vec![vec![1]]);
    }

    #[test]
    fn test_four_elements_count() {
        let result = permute(vec![1, 2, 3, 4]);
        assert_eq!(result.len(), 24); // 4! = 24
    }

    #[test]
    fn test_negative_numbers() {
        let result = sorted(permute(vec![-1, 0, 1]));
        assert_eq!(result.len(), 6);
        assert!(result.contains(&vec![-1, 0, 1]));
        assert!(result.contains(&vec![1, 0, -1]));
    }

    #[test]
    fn test_all_permutations_unique() {
        let result = permute(vec![1, 2, 3, 4]);
        let mut unique = result.clone();
        unique.sort();
        unique.dedup();
        assert_eq!(result.len(), unique.len());
    }
}
