pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
    // Implement your solution here
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_board(rows: Vec<&str>) -> Vec<Vec<char>> {
        rows.into_iter().map(|r| r.chars().collect()).collect()
    }

    #[test]
    fn test_example_1() {
        let board = make_board(vec!["ABCE", "SFCS", "ADEE"]);
        assert_eq!(exist(board, "ABCCED".to_string()), true);
    }

    #[test]
    fn test_example_2() {
        let board = make_board(vec!["ABCE", "SFCS", "ADEE"]);
        assert_eq!(exist(board, "SEE".to_string()), true);
    }

    #[test]
    fn test_example_3() {
        let board = make_board(vec!["ABCE", "SFCS", "ADEE"]);
        assert_eq!(exist(board, "ABCB".to_string()), false);
    }

    #[test]
    fn test_single_cell_found() {
        let board = make_board(vec!["A"]);
        assert_eq!(exist(board, "A".to_string()), true);
    }

    #[test]
    fn test_single_cell_not_found() {
        let board = make_board(vec!["A"]);
        assert_eq!(exist(board, "B".to_string()), false);
    }

    #[test]
    fn test_full_board() {
        let board = make_board(vec!["AB", "CD"]);
        assert_eq!(exist(board, "ABDC".to_string()), true);
    }

    #[test]
    fn test_word_longer_than_board() {
        let board = make_board(vec!["AB", "CD"]);
        assert_eq!(exist(board, "ABCDE".to_string()), false);
    }
}
