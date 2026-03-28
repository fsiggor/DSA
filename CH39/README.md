# Construct Binary Tree from Preorder and Inorder Traversal

## Difficulty: Medium

## Description

Given two integer arrays `preorder` and `inorder` where `preorder` is the preorder traversal of a binary tree and `inorder` is the inorder traversal of the same tree, construct and return the binary tree in level-order `Vec<Option<i32>>` format.

## Examples

**Example 1:**
```
Input: preorder = [3, 9, 20, 15, 7], inorder = [9, 3, 15, 20, 7]
Output: [Some(3), Some(9), Some(20), None, None, Some(15), Some(7)]
```

**Example 2:**
```
Input: preorder = [-1], inorder = [-1]
Output: [Some(-1)]
```

## Constraints

- `1 <= preorder.len() <= 3000`
- `inorder.len() == preorder.len()`
- `-3000 <= preorder[i], inorder[i] <= 3000`
- `preorder` and `inorder` consist of unique values.
- Each value of `inorder` also appears in `preorder`.
- `preorder` is guaranteed to be the preorder traversal of the tree.
- `inorder` is guaranteed to be the inorder traversal of the tree.

## Hint

The first element in preorder is the root. Find its position in inorder to split into left and right subtrees, then recurse.
