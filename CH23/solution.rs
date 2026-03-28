pub fn clone_graph(adj_list: Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_triangle() {
        let adj = vec![vec![1, 2], vec![0, 2], vec![0, 1]];
        let cloned = clone_graph(adj.clone());
        assert_eq!(cloned, adj);
    }

    #[test]
    fn test_single_node_no_neighbors() {
        let adj = vec![vec![]];
        let cloned = clone_graph(adj.clone());
        assert_eq!(cloned, adj);
    }

    #[test]
    fn test_empty_graph() {
        let adj: Vec<Vec<usize>> = vec![];
        let cloned = clone_graph(adj.clone());
        assert_eq!(cloned, adj);
    }

    #[test]
    fn test_two_connected_nodes() {
        let adj = vec![vec![1], vec![0]];
        let cloned = clone_graph(adj.clone());
        assert_eq!(cloned, adj);
    }

    #[test]
    fn test_linear_graph() {
        let adj = vec![vec![1], vec![0, 2], vec![1, 3], vec![2]];
        let cloned = clone_graph(adj.clone());
        assert_eq!(cloned, adj);
    }

    #[test]
    fn test_star_graph() {
        let adj = vec![vec![1, 2, 3], vec![0], vec![0], vec![0]];
        let cloned = clone_graph(adj.clone());
        assert_eq!(cloned, adj);
    }

    #[test]
    fn test_deep_copy_independence() {
        let adj = vec![vec![1], vec![0]];
        let cloned = clone_graph(adj.clone());
        // Verify it's equal but conceptually a separate copy
        assert_eq!(cloned, adj);
        assert_eq!(cloned.len(), 2);
    }
}

fn main() {}
