# Trapping Rain Water

## Difficulty: Hard

## Description

Given `n` non-negative integers representing an elevation map where the width of each bar is 1, compute how much water it can trap after raining.

## Examples

**Example 1:**
```
Input: height = [0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]
Output: 6
Explanation: The elevation map traps 6 units of rain water.
```

**Example 2:**
```
Input: height = [4, 2, 0, 3, 2, 5]
Output: 9
```

**Example 3:**
```
Input: height = [1, 0, 1]
Output: 1
```

## Constraints

- `0 <= height.len() <= 2 * 10^4`
- `0 <= height[i] <= 10^5`

## Hint

Use two pointers from both ends. Maintain the maximum height seen from the left and right. At each step, move the pointer with the smaller max height inward and accumulate the trapped water as the difference between the max height and the current height.
