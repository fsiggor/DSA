# Maximum Depth of Binary Tree

## Difficulty: Easy

## Description

Given the root of a binary tree, return its **maximum depth**.

The maximum depth is the number of nodes along the longest path from the root down to the farthest leaf node.

For simplicity, we represent the tree as a level-order vector where:
- Index `0` is the root.
- For the node at index `i`, the left child is at `2*i + 1` and the right child is at `2*i + 2`.
- `None` indicates the absence of a node.

## Examples

**Example 1:**
```
Input: tree = [Some(3), Some(9), Some(20), None, None, Some(15), Some(7)]
Output: 3
```

**Example 2:**
```
Input: tree = [Some(1), None, Some(2)]
Output: 2
```

## Constraints

- The number of nodes is in the range `[0, 10^4]`.
- `-100 <= Node.val <= 100`

## Hint

Use recursion or BFS. In an array representation, depth can be calculated based on the indices of the last non-None elements.
