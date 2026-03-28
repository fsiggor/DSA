pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sorted(mut v: Vec<Vec<String>>) -> Vec<Vec<String>> {
        v.sort();
        v
    }

    #[test]
    fn test_n_equals_4() {
        let result = sorted(solve_n_queens(4));
        let expected = sorted(vec![
            vec![
                ".Q..".to_string(),
                "...Q".to_string(),
                "Q...".to_string(),
                "..Q.".to_string(),
            ],
            vec![
                "..Q.".to_string(),
                "Q...".to_string(),
                "...Q".to_string(),
                ".Q..".to_string(),
            ],
        ]);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_n_equals_1() {
        let result = solve_n_queens(1);
        assert_eq!(result, vec![vec!["Q".to_string()]]);
    }

    #[test]
    fn test_n_equals_2() {
        let result = solve_n_queens(2);
        assert!(result.is_empty());
    }

    #[test]
    fn test_n_equals_3() {
        let result = solve_n_queens(3);
        assert!(result.is_empty());
    }

    #[test]
    fn test_n_equals_5() {
        let result = solve_n_queens(5);
        assert_eq!(result.len(), 10);
    }

    #[test]
    fn test_n_equals_8() {
        let result = solve_n_queens(8);
        assert_eq!(result.len(), 92);
    }

    #[test]
    fn test_board_dimensions() {
        let result = solve_n_queens(4);
        for board in &result {
            assert_eq!(board.len(), 4);
            for row in board {
                assert_eq!(row.len(), 4);
            }
        }
    }

    #[test]
    fn test_one_queen_per_row() {
        let result = solve_n_queens(4);
        for board in &result {
            for row in board {
                assert_eq!(row.chars().filter(|&c| c == 'Q').count(), 1);
            }
        }
    }
}
