# Remove Nth Node From End of List

## Difficulty: Medium

## Description

Given a list represented as a vector and an integer `n`, remove the `n`th node from the **end** of the list and return the modified list.

## Examples

**Example 1:**
```
Input: list = [1, 2, 3, 4, 5], n = 2
Output: [1, 2, 3, 5]
Explanation: The 2nd node from the end is 4, so it is removed.
```

**Example 2:**
```
Input: list = [1], n = 1
Output: []
```

**Example 3:**
```
Input: list = [1, 2], n = 1
Output: [1]
```

## Constraints

- `1 <= list.len() <= 30`
- `0 <= list[i] <= 100`
- `1 <= n <= list.len()`

## Hint

Calculate the index to remove: it is `list.len() - n`. Alternatively, use two pointers separated by `n` positions to find the element in one pass.
