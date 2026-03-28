pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sorted(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        intervals.sort();
        intervals
    }

    #[test]
    fn test_example_1() {
        let input = vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]];
        let expected = vec![vec![1, 6], vec![8, 10], vec![15, 18]];
        assert_eq!(sorted(merge(input)), sorted(expected));
    }

    #[test]
    fn test_example_2() {
        let input = vec![vec![1, 4], vec![4, 5]];
        let expected = vec![vec![1, 5]];
        assert_eq!(sorted(merge(input)), sorted(expected));
    }

    #[test]
    fn test_single_interval() {
        let input = vec![vec![1, 5]];
        assert_eq!(merge(input), vec![vec![1, 5]]);
    }

    #[test]
    fn test_no_overlap() {
        let input = vec![vec![1, 2], vec![4, 5], vec![7, 8]];
        let expected = vec![vec![1, 2], vec![4, 5], vec![7, 8]];
        assert_eq!(sorted(merge(input)), sorted(expected));
    }

    #[test]
    fn test_all_overlap() {
        let input = vec![vec![1, 4], vec![2, 5], vec![3, 6]];
        let expected = vec![vec![1, 6]];
        assert_eq!(merge(input), expected);
    }

    #[test]
    fn test_nested_intervals() {
        let input = vec![vec![1, 10], vec![2, 5], vec![3, 7]];
        let expected = vec![vec![1, 10]];
        assert_eq!(merge(input), expected);
    }

    #[test]
    fn test_unsorted_input() {
        let input = vec![vec![8, 10], vec![1, 3], vec![2, 6], vec![15, 18]];
        let expected = vec![vec![1, 6], vec![8, 10], vec![15, 18]];
        assert_eq!(sorted(merge(input)), sorted(expected));
    }

    #[test]
    fn test_adjacent_intervals() {
        let input = vec![vec![1, 2], vec![3, 4]];
        let expected = vec![vec![1, 2], vec![3, 4]];
        assert_eq!(sorted(merge(input)), sorted(expected));
    }

    #[test]
    fn test_same_intervals() {
        let input = vec![vec![1, 3], vec![1, 3]];
        let expected = vec![vec![1, 3]];
        assert_eq!(merge(input), expected);
    }
}
