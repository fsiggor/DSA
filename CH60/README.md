# Find Minimum in Rotated Sorted Array

## Difficulty: Medium

## Description

Suppose an array of length `n` sorted in ascending order is rotated between `1` and `n` times. For example, the array `[0, 1, 2, 4, 5, 6, 7]` might become `[4, 5, 6, 7, 0, 1, 2]` after rotating 4 times.

Given the sorted rotated array `nums` of **unique** elements, return the minimum element of this array.

You must write an algorithm that runs in O(log n) time.

## Examples

**Example 1:**
```
Input: nums = [3, 4, 5, 1, 2]
Output: 1
Explanation: The original array was [1, 2, 3, 4, 5] rotated 3 times.
```

**Example 2:**
```
Input: nums = [4, 5, 6, 7, 0, 1, 2]
Output: 0
```

**Example 3:**
```
Input: nums = [11, 13, 15, 17]
Output: 11
Explanation: The array was not rotated (or rotated n times).
```

## Constraints

- `1 <= nums.len() <= 5000`
- `-5000 <= nums[i] <= 5000`
- All values in `nums` are **unique**
- `nums` is a sorted array that has been rotated between 1 and n times

## Hint

Use binary search. Compare the middle element with the rightmost element: if the middle is greater, the minimum is in the right half; otherwise, it is in the left half (including mid).
