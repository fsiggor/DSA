# Top K Frequent Elements

## Difficulty: Medium

## Description

Given an integer array `nums` and an integer `k`, return the `k` most frequent elements. You may return the answer in any order.

## Examples

**Example 1:**
```
Input: nums = [1, 1, 1, 2, 2, 3], k = 2
Output: [1, 2]
```

**Example 2:**
```
Input: nums = [1], k = 1
Output: [1]
```

## Constraints

- `1 <= nums.len() <= 10^5`
- `-10^4 <= nums[i] <= 10^4`
- `k` is in the range `[1, number of unique elements]`.
- The answer is guaranteed to be unique.

## Hint

Count frequencies using a HashMap, then use bucket sort or a heap to find the top k elements. Bucket sort gives O(n) time complexity.
