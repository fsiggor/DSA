# Rotate Array

## Difficulty: Medium

## Description

Given an integer array `nums`, rotate the array to the right by `k` steps, where `k` is non-negative.

The rotation must be done **in-place** with O(1) extra space.

## Examples

**Example 1:**
```
Input: nums = [1, 2, 3, 4, 5, 6, 7], k = 3
Output: [5, 6, 7, 1, 2, 3, 4]
Explanation:
  rotate 1 step to the right: [7, 1, 2, 3, 4, 5, 6]
  rotate 2 steps to the right: [6, 7, 1, 2, 3, 4, 5]
  rotate 3 steps to the right: [5, 6, 7, 1, 2, 3, 4]
```

**Example 2:**
```
Input: nums = [-1, -100, 3, 99], k = 2
Output: [3, 99, -1, -100]
```

**Example 3:**
```
Input: nums = [1, 2], k = 3
Output: [2, 1]
```

## Constraints

- `1 <= nums.len() <= 10^5`
- `-2^31 <= nums[i] <= 2^31 - 1`
- `0 <= k <= 10^5`

## Hint

Use the reversal algorithm: first reverse the entire array, then reverse the first `k % n` elements, then reverse the remaining elements. This achieves rotation in O(n) time and O(1) space.
