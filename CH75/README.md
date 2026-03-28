# Merge Two Sorted Lists

## Difficulty: Easy

## Description

Given the heads of two sorted linked lists `list1` and `list2`, merge the two lists into one sorted list.

The resulting list should be made by splicing together the nodes of the two input lists.

Return the head of the merged list.

For simplicity, we use vectors instead of actual linked lists in Rust.

## Examples

**Example 1:**
```
Input: list1 = [1, 2, 4], list2 = [1, 3, 4]
Output: [1, 1, 2, 3, 4, 4]
```

**Example 2:**
```
Input: list1 = [], list2 = []
Output: []
```

**Example 3:**
```
Input: list1 = [], list2 = [0]
Output: [0]
```

## Constraints

- The number of nodes in each list is in the range `[0, 50]`.
- `-100 <= Node.val <= 100`
- Both lists are sorted in non-decreasing order.

## Hint

Compare elements from both lists and append the smaller one to the result.
