# Climbing Stairs

## Difficulty: Easy

## Description

You are climbing a staircase. It takes `n` steps to reach the top.

Each time you can climb either **1** or **2** steps. In how many distinct ways can you climb to the top?

## Examples

**Example 1:**
```
Input: n = 2
Output: 2
Explanation: There are two ways to climb to the top:
1. 1 step + 1 step
2. 2 steps
```

**Example 2:**
```
Input: n = 3
Output: 3
Explanation: There are three ways to climb to the top:
1. 1 + 1 + 1
2. 1 + 2
3. 2 + 1
```

## Constraints

- `1 <= n <= 45`

## Hint

This problem is equivalent to the Fibonacci sequence. The number of ways to reach step `n` is the sum of ways to reach steps `n-1` and `n-2`.
