pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_island() {
        let grid = vec![
            vec!['1','1','1','1','0'],
            vec!['1','1','0','1','0'],
            vec!['1','1','0','0','0'],
            vec!['0','0','0','0','0'],
        ];
        assert_eq!(num_islands(grid), 1);
    }

    #[test]
    fn test_three_islands() {
        let grid = vec![
            vec!['1','1','0','0','0'],
            vec!['1','1','0','0','0'],
            vec!['0','0','1','0','0'],
            vec!['0','0','0','1','1'],
        ];
        assert_eq!(num_islands(grid), 3);
    }

    #[test]
    fn test_all_water() {
        let grid = vec![
            vec!['0','0','0'],
            vec!['0','0','0'],
        ];
        assert_eq!(num_islands(grid), 0);
    }

    #[test]
    fn test_all_land() {
        let grid = vec![
            vec!['1','1'],
            vec!['1','1'],
        ];
        assert_eq!(num_islands(grid), 1);
    }

    #[test]
    fn test_single_cell_land() {
        let grid = vec![vec!['1']];
        assert_eq!(num_islands(grid), 1);
    }

    #[test]
    fn test_single_cell_water() {
        let grid = vec![vec!['0']];
        assert_eq!(num_islands(grid), 0);
    }

    #[test]
    fn test_diagonal_not_connected() {
        let grid = vec![
            vec!['1','0'],
            vec!['0','1'],
        ];
        assert_eq!(num_islands(grid), 2);
    }

    #[test]
    fn test_checkerboard() {
        let grid = vec![
            vec!['1','0','1'],
            vec!['0','1','0'],
            vec!['1','0','1'],
        ];
        assert_eq!(num_islands(grid), 5);
    }
}

fn main() {}
