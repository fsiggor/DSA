# Lowest Common Ancestor of a Binary Search Tree

## Difficulty: Medium

## Description

Given a binary search tree (BST) represented as a level-order `Vec<Option<i32>>`, find the lowest common ancestor (LCA) of two given nodes `p` and `q` in the BST.

The lowest common ancestor is the deepest node that has both `p` and `q` as descendants (where a node can be a descendant of itself).

## Examples

**Example 1:**
```
Input: tree = [Some(6), Some(2), Some(8), Some(0), Some(4), Some(7), Some(9), None, None, Some(3), Some(5)], p = 2, q = 8
Output: 6
Explanation: The LCA of nodes 2 and 8 is 6.
```

**Example 2:**
```
Input: tree = [Some(6), Some(2), Some(8), Some(0), Some(4), Some(7), Some(9), None, None, Some(3), Some(5)], p = 2, q = 4
Output: 2
Explanation: The LCA of nodes 2 and 4 is 2, since a node can be a descendant of itself.
```

## Constraints

- The number of nodes in the tree is in the range `[2, 10^5]`.
- `-10^9 <= Node.val <= 10^9`
- All node values are unique.
- `p != q`
- `p` and `q` are guaranteed to exist in the BST.

## Hint

Leverage the BST property: if both p and q are smaller than the current node, go left; if both are larger, go right; otherwise, the current node is the LCA.
