# Next Permutation

## Difficulty: Medium

## Description

A permutation of an array of integers is an arrangement of its members into a sequence or linear order.

The **next permutation** of an array of integers is the next lexicographically greater permutation of its integer. If the array is the last permutation (sorted in descending order), rearrange it as the lowest possible order (sorted in ascending order).

The replacement must be **in place** and use only constant extra memory.

## Examples

**Example 1:**
```
Input: nums = [1, 2, 3]
Output: [1, 3, 2]
```

**Example 2:**
```
Input: nums = [3, 2, 1]
Output: [1, 2, 3]
Explanation: This is the last permutation, so we return the first permutation.
```

**Example 3:**
```
Input: nums = [1, 1, 5]
Output: [1, 5, 1]
```

**Example 4:**
```
Input: nums = [1, 3, 2]
Output: [2, 1, 3]
```

## Constraints

- `1 <= nums.len() <= 100`
- `0 <= nums[i] <= 100`

## Hint

Find the largest index `i` such that `nums[i] < nums[i + 1]`. If no such index exists, reverse the whole array. Otherwise, find the largest index `j > i` such that `nums[i] < nums[j]`, swap them, and reverse the suffix after index `i`.
