# Largest Rectangle in Histogram

## Difficulty: Hard

## Description

Given a vector of integers `heights` representing the histogram's bar heights where the width of each bar is `1`, return the area of the largest rectangle in the histogram.

## Examples

**Example 1:**
```
Input: heights = [2, 1, 5, 6, 2, 3]
Output: 10
Explanation: The largest rectangle has area = 5 * 2 = 10 (bars at indices 2 and 3).
```

**Example 2:**
```
Input: heights = [2, 4]
Output: 4
```

## Constraints

- `1 <= heights.len() <= 10^5`
- `0 <= heights[i] <= 10^4`

## Hint

Use a monotonic increasing stack of indices. For each bar, while the stack's top bar is taller, pop it and calculate the rectangle width using the current index and the new stack top.
