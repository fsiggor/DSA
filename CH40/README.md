# First Missing Positive

## Difficulty: Hard

## Description

Given an unsorted integer array `nums`, return the smallest missing positive integer.

You must implement an algorithm that runs in O(n) time and uses O(1) auxiliary space.

## Examples

**Example 1:**
```
Input: nums = [1, 2, 0]
Output: 3
Explanation: The numbers 1 and 2 are present, so the first missing positive is 3.
```

**Example 2:**
```
Input: nums = [3, 4, -1, 1]
Output: 2
Explanation: 1 is present but 2 is missing.
```

**Example 3:**
```
Input: nums = [7, 8, 9, 11, 12]
Output: 1
Explanation: The smallest positive integer 1 is missing.
```

**Example 4:**
```
Input: nums = [1]
Output: 2
```

## Constraints

- `1 <= nums.len() <= 10^5`
- `-2^31 <= nums[i] <= 2^31 - 1`

## Hint

Use the array itself as a hash map by placing each positive number `x` (where `1 <= x <= n`) at index `x - 1`. Then scan the array to find the first position where `nums[i] != i + 1`.
