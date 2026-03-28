# Invert Binary Tree

## Difficulty: Easy

## Description

Given the root of a binary tree, invert the tree (mirror it) and return the result.

Inverting a tree means swapping the left and right children of every node, recursively.

For simplicity, we use a level-order vector representation where `None` indicates the absence of a node.

## Examples

**Example 1:**
```
Input:  [4, 2, 7, 1, 3, 6, 9]
Output: [4, 7, 2, 9, 6, 3, 1]
```

**Example 2:**
```
Input:  [2, 1, 3]
Output: [2, 3, 1]
```

**Example 3:**
```
Input:  []
Output: []
```

## Constraints

- The number of nodes is in the range `[0, 100]`.
- `-100 <= Node.val <= 100`

## Hint

Recursively swap left and right children. In the array representation, for each level, reverse the order of nodes.
