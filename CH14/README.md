# Validate Binary Search Tree

## Difficulty: Medium

## Description

Given a binary tree represented as a level-order vector of `Option<i32>` values, determine if it is a valid binary search tree (BST).

A valid BST is defined as follows:
- The left subtree of a node contains only nodes with keys **less than** the node's key.
- The right subtree of a node contains only nodes with keys **greater than** the node's key.
- Both the left and right subtrees must also be binary search trees.

The tree is given in level-order traversal where `None` represents a missing node.

## Examples

**Example 1:**
```
Input: tree = [Some(2), Some(1), Some(3)]
Output: true
```

**Example 2:**
```
Input: tree = [Some(5), Some(1), Some(4), None, None, Some(3), Some(6)]
Output: false
Explanation: The root's right child is 4, which is less than 5.
```

**Example 3:**
```
Input: tree = [Some(1)]
Output: true
```

## Constraints

- `1 <= tree.len() <= 10^4`
- `-2^31 <= node.val <= 2^31 - 1`

## Hint

Use recursive validation with min/max bounds. Each node must fall within a valid range determined by its ancestors. Parse the level-order vector to identify parent-child relationships using index math: for a node at index `i`, its left child is at `2*i + 1` and right child at `2*i + 2`.
