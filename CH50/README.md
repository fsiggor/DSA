# Symmetric Tree

## Difficulty: Easy

## Description

Given a binary tree represented as a level-order vector of `Option<i32>` values, check whether it is a mirror of itself (i.e., symmetric around its center).

A tree is symmetric if the left subtree is a mirror reflection of the right subtree.

## Examples

**Example 1:**
```
Input: tree = [Some(1), Some(2), Some(2), Some(3), Some(4), Some(4), Some(3)]
Output: true
Explanation: The tree is symmetric around its center.
```

**Example 2:**
```
Input: tree = [Some(1), Some(2), Some(2), None, Some(3), None, Some(3)]
Output: false
Explanation: The right child of the left subtree does not mirror the left child of the right subtree.
```

**Example 3:**
```
Input: tree = [Some(1)]
Output: true
```

## Constraints

- `1 <= tree.len() <= 1000`
- `-100 <= node.val <= 100`

## Hint

Compare nodes level by level: the left child of the left subtree should equal the right child of the right subtree, and vice versa. Use index math on the level-order array, or recursively compare mirror positions.
