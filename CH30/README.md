# 85. Power of Two

**Difficulty:** Easy

**Category:** Bit Manipulation

## Description

Given an integer `n`, return `true` if it is a power of two. Otherwise, return `false`.

An integer `n` is a power of two if there exists an integer `x` such that `n == 2^x`.

## Examples

**Example 1:**

```
Input: n = 1
Output: true
Explanation: 2^0 = 1
```

**Example 2:**

```
Input: n = 16
Output: true
Explanation: 2^4 = 16
```

**Example 3:**

```
Input: n = 3
Output: false
```

## Constraints

- `-2^31 <= n <= 2^31 - 1`

## Hint

A power of two in binary has exactly one `1` bit. Use the trick `n & (n - 1) == 0` to check if only one bit is set. Be careful to handle `n <= 0` since negative numbers and zero are never powers of two.
