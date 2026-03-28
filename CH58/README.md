# Kth Smallest Element in a BST

## Difficulty: Medium

## Description

Given the root of a binary search tree represented as a level-order `Vec<Option<i32>>` and an integer `k`, return the `k`th smallest value (1-indexed) of all the values of the nodes in the tree.

## Examples

**Example 1:**
```
Input: tree = [Some(3), Some(1), Some(4), None, Some(2)], k = 1
Output: 1
```

**Example 2:**
```
Input: tree = [Some(5), Some(3), Some(6), Some(2), Some(4), None, None, Some(1)], k = 3
Output: 3
```

## Constraints

- The number of nodes in the tree is `n`.
- `1 <= k <= n <= 10^4`
- `0 <= Node.val <= 10^4`

## Hint

Perform an inorder traversal of the BST, which visits nodes in ascending order. Return the kth element visited.
