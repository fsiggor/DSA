# Search in Rotated Sorted Array

## Difficulty: Medium

## Description

There is an integer array `nums` sorted in ascending order (with **distinct** values) that has been possibly rotated at an unknown pivot index.

Given the array `nums` after the possible rotation and an integer `target`, return the index of `target` if it is in `nums`, or `-1` if it is not in `nums`.

You must write an algorithm with O(log n) runtime complexity.

## Examples

**Example 1:**
```
Input: nums = [4, 5, 6, 7, 0, 1, 2], target = 0
Output: 4
```

**Example 2:**
```
Input: nums = [4, 5, 6, 7, 0, 1, 2], target = 3
Output: -1
```

**Example 3:**
```
Input: nums = [1], target = 0
Output: -1
```

**Example 4:**
```
Input: nums = [1], target = 1
Output: 0
```

## Constraints

- `1 <= nums.len() <= 5000`
- `-10^4 <= nums[i] <= 10^4`
- All values of `nums` are **unique**
- `nums` is a sorted array that may have been rotated
- `-10^4 <= target <= 10^4`

## Hint

Use modified binary search. At each step, one half of the array is always sorted. Determine which half is sorted and check if the target lies within that sorted half to decide which direction to search.
