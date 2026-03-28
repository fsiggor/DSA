pub fn pacific_atlantic(heights: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sort_result(mut result: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        result.sort();
        result
    }

    #[test]
    fn test_example_1() {
        let heights = vec![
            vec![1, 2, 2, 3, 5],
            vec![3, 2, 3, 4, 4],
            vec![2, 4, 5, 3, 1],
            vec![6, 7, 1, 4, 5],
            vec![5, 1, 1, 2, 4],
        ];
        let expected = vec![
            vec![0, 4],
            vec![1, 3],
            vec![1, 4],
            vec![2, 2],
            vec![3, 0],
            vec![3, 1],
            vec![4, 0],
        ];
        assert_eq!(sort_result(pacific_atlantic(heights)), sort_result(expected));
    }

    #[test]
    fn test_single_cell() {
        let heights = vec![vec![1]];
        assert_eq!(pacific_atlantic(heights), vec![vec![0, 0]]);
    }

    #[test]
    fn test_single_row() {
        let heights = vec![vec![1, 2, 3]];
        let result = sort_result(pacific_atlantic(heights));
        // All cells in a single row touch both oceans
        let expected = sort_result(vec![vec![0, 0], vec![0, 1], vec![0, 2]]);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_single_column() {
        let heights = vec![vec![1], vec![2], vec![3]];
        let result = sort_result(pacific_atlantic(heights));
        let expected = sort_result(vec![vec![0, 0], vec![1, 0], vec![2, 0]]);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_uniform_heights() {
        let heights = vec![
            vec![1, 1],
            vec![1, 1],
        ];
        let result = sort_result(pacific_atlantic(heights));
        let expected = sort_result(vec![
            vec![0, 0], vec![0, 1],
            vec![1, 0], vec![1, 1],
        ]);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_descending_grid() {
        let heights = vec![
            vec![5, 4, 3],
            vec![4, 3, 2],
            vec![3, 2, 1],
        ];
        let result = sort_result(pacific_atlantic(heights));
        // Top-left corner can reach Pacific easily, Atlantic via decreasing path
        assert!(result.contains(&vec![0, 0]));
    }
}
