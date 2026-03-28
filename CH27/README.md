# Daily Temperatures

## Difficulty: Medium

## Description

Given a vector of integers `temps` representing daily temperatures, return a vector where each element tells you how many days you would have to wait until a warmer temperature. If there is no future day with a warmer temperature, put `0` for that day.

## Examples

**Example 1:**
```
Input: temps = [73, 74, 75, 71, 69, 72, 76, 73]
Output: [1, 1, 4, 2, 1, 1, 0, 0]
```

**Example 2:**
```
Input: temps = [30, 40, 50, 60]
Output: [1, 1, 1, 0]
```

**Example 3:**
```
Input: temps = [30, 60, 90]
Output: [1, 1, 0]
```

## Constraints

- `1 <= temps.len() <= 10^5`
- `30 <= temps[i] <= 100`

## Hint

Use a monotonic decreasing stack that stores indices. For each new temperature, pop all stack entries with a lower temperature and compute the difference in indices.
