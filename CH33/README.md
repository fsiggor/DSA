# Binary Tree Level Order Traversal

## Difficulty: Medium

## Description

Given a binary tree represented as a level-order `Vec<Option<i32>>`, return the level order traversal of its nodes' values (i.e., from left to right, level by level).

## Examples

**Example 1:**
```
Input: tree = [Some(3), Some(9), Some(20), None, None, Some(15), Some(7)]
Output: [[3], [9, 20], [15, 7]]
```

**Example 2:**
```
Input: tree = [Some(1)]
Output: [[1]]
```

**Example 3:**
```
Input: tree = []
Output: []
```

## Constraints

- `0 <= number of nodes <= 2000`
- `-1000 <= Node.val <= 1000`
- The tree is given in level-order format where `None` represents a missing node.

## Hint

Use a queue (BFS) to process nodes level by level. Track the number of nodes at each level to group them correctly.
