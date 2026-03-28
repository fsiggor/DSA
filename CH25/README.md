# Binary Tree Right Side View

## Difficulty: Medium

## Description

Given the root of a binary tree represented as a level-order `Vec<Option<i32>>`, imagine yourself standing on the right side of it. Return the values of the nodes you can see ordered from top to bottom.

## Examples

**Example 1:**
```
Input: tree = [Some(1), Some(2), Some(3), None, Some(5), None, Some(4)]
Output: [1, 3, 4]
```

**Example 2:**
```
Input: tree = [Some(1), None, Some(3)]
Output: [1, 3]
```

**Example 3:**
```
Input: tree = []
Output: []
```

## Constraints

- The number of nodes in the tree is in the range `[0, 100]`.
- `-100 <= Node.val <= 100`

## Hint

Use BFS (level order traversal) and take the last node at each level. Alternatively, use DFS visiting right subtree first.
