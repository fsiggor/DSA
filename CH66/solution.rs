pub fn serialize(tree: Vec<Option<i32>>) -> String {
    // Implement your solution here
    todo!()
}

pub fn deserialize(data: String) -> Vec<Option<i32>> {
    // Implement your solution here
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_roundtrip_example_1() {
        let tree = vec![Some(1), Some(2), Some(3), None, None, Some(4), Some(5)];
        let serialized = serialize(tree.clone());
        let deserialized = deserialize(serialized);
        assert_eq!(deserialized, tree);
    }

    #[test]
    fn test_roundtrip_empty() {
        let tree: Vec<Option<i32>> = vec![];
        let serialized = serialize(tree.clone());
        let deserialized = deserialize(serialized);
        assert_eq!(deserialized, tree);
    }

    #[test]
    fn test_roundtrip_single_node() {
        let tree = vec![Some(42)];
        let serialized = serialize(tree.clone());
        let deserialized = deserialize(serialized);
        assert_eq!(deserialized, tree);
    }

    #[test]
    fn test_roundtrip_complete_tree() {
        let tree = vec![Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7)];
        let serialized = serialize(tree.clone());
        let deserialized = deserialize(serialized);
        assert_eq!(deserialized, tree);
    }

    #[test]
    fn test_roundtrip_left_skewed() {
        let tree = vec![Some(1), Some(2), None, Some(3)];
        let serialized = serialize(tree.clone());
        let deserialized = deserialize(serialized);
        assert_eq!(deserialized, tree);
    }

    #[test]
    fn test_roundtrip_negative_values() {
        let tree = vec![Some(-1), Some(-2), Some(-3)];
        let serialized = serialize(tree.clone());
        let deserialized = deserialize(serialized);
        assert_eq!(deserialized, tree);
    }

    #[test]
    fn test_serialize_produces_string() {
        let tree = vec![Some(1), Some(2), Some(3)];
        let serialized = serialize(tree);
        assert!(!serialized.is_empty());
    }

    #[test]
    fn test_roundtrip_sparse_tree() {
        let tree = vec![Some(3), Some(9), Some(20), None, None, Some(15), Some(7)];
        let serialized = serialize(tree.clone());
        let deserialized = deserialize(serialized);
        assert_eq!(deserialized, tree);
    }
}
