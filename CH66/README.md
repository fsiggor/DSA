# Serialize and Deserialize Binary Tree

## Difficulty: Hard

## Description

Design an algorithm to serialize a binary tree (given as a level-order `Vec<Option<i32>>`) into a string, and deserialize that string back into the same level-order `Vec<Option<i32>>` representation.

The serialization/deserialization algorithm must be reversible: deserializing a serialized tree must produce the original tree.

## Examples

**Example 1:**
```
Input: tree = [Some(1), Some(2), Some(3), None, None, Some(4), Some(5)]
Serialized: "1,2,3,null,null,4,5"
Deserialized: [Some(1), Some(2), Some(3), None, None, Some(4), Some(5)]
```

**Example 2:**
```
Input: tree = []
Serialized: ""
Deserialized: []
```

## Constraints

- The number of nodes in the tree is in the range `[0, 10^4]`.
- `-1000 <= Node.val <= 1000`
- The output of `deserialize(serialize(tree))` must equal the original `tree`.

## Hint

Use a level-order (BFS) approach for serialization, encoding None values as "null". For deserialization, parse the string and reconstruct the tree level by level.
