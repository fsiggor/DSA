# Remove Duplicates from Sorted Array

## Difficulty: Easy

## Description

Given an integer array `nums` sorted in non-decreasing order, remove the duplicates **in-place** such that each unique element appears only once. The relative order of the elements should be kept the same.

Return the number of unique elements. The first `k` elements of `nums` should contain the unique elements in their original order. The remaining elements of `nums` are not important.

## Examples

**Example 1:**
```
Input: nums = [1, 1, 2]
Output: 2, nums = [1, 2, _]
Explanation: The function returns k = 2, with the first two elements being 1 and 2.
```

**Example 2:**
```
Input: nums = [0, 0, 1, 1, 1, 2, 2, 3, 3, 4]
Output: 5, nums = [0, 1, 2, 3, 4, _, _, _, _, _]
Explanation: The function returns k = 5, with the first five elements being 0, 1, 2, 3, and 4.
```

**Example 3:**
```
Input: nums = [1]
Output: 1, nums = [1]
```

## Constraints

- `1 <= nums.len() <= 3 * 10^4`
- `-100 <= nums[i] <= 100`
- `nums` is sorted in non-decreasing order

## Hint

Use a two-pointer approach: a slow pointer tracks the position to place the next unique element, while a fast pointer scans through the array looking for new unique values.
