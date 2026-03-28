# Unique Paths

**Difficulty:** Medium

**Category:** Dynamic Programming

## Description

A robot is located at the top-left corner of an `m x n` grid. The robot can only move either down or right at any point in time. The robot is trying to reach the bottom-right corner of the grid.

How many possible unique paths are there?

## Examples

**Example 1:**

```
Input: m = 3, n = 7
Output: 28
```

**Example 2:**

```
Input: m = 3, n = 2
Output: 3
Explanation: From the top-left corner, there are a total of 3 ways to reach the bottom-right corner:
1. Right -> Down -> Down
2. Down -> Down -> Right
3. Down -> Right -> Down
```

**Example 3:**

```
Input: m = 1, n = 1
Output: 1
```

## Constraints

- `1 <= m, n <= 100`

## Hint

The number of paths to any cell is the sum of the paths to the cell above it and the cell to its left. Use a 2D DP table or optimize to a 1D array.
