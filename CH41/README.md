# Binary Search

## Difficulty: Easy

## Description

Given an array of integers `nums` sorted in ascending order and an integer `target`, write a function to search `target` in `nums`. If `target` exists, return its index. Otherwise, return `-1`.

You must implement an algorithm with O(log n) time complexity.

## Examples

**Example 1:**
```
Input: nums = [-1, 0, 3, 5, 9, 12], target = 9
Output: 4
Explanation: 9 exists in nums and its index is 4.
```

**Example 2:**
```
Input: nums = [-1, 0, 3, 5, 9, 12], target = 2
Output: -1
Explanation: 2 does not exist in nums, so return -1.
```

## Constraints

- `1 <= nums.len() <= 10^4`
- All elements of `nums` are unique.
- `nums` is sorted in ascending order.

## Hint

Maintain two pointers (left and right) and compare the middle element with the target to decide which half to continue searching.
