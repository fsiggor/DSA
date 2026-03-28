# 88. Reverse Integer

**Difficulty:** Medium

**Category:** Math

## Description

Given a signed 32-bit integer `x`, return `x` with its digits reversed. If reversing `x` causes the value to go outside the signed 32-bit integer range `[-2^31, 2^31 - 1]`, then return `0`.

Assume the environment does not allow you to store 64-bit integers (signed or unsigned).

## Examples

**Example 1:**

```
Input: x = 123
Output: 321
```

**Example 2:**

```
Input: x = -123
Output: -321
```

**Example 3:**

```
Input: x = 120
Output: 21
```

## Constraints

- `-2^31 <= x <= 2^31 - 1`

## Hint

Build the reversed number digit by digit using modulo and division. Before adding each digit, check whether the result would overflow by comparing against `i32::MAX / 10` and `i32::MIN / 10`.
