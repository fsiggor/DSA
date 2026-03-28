# Evaluate Reverse Polish Notation

## Difficulty: Medium

## Description

You are given an array of strings `tokens` that represents an arithmetic expression in Reverse Polish Notation (RPN).

Evaluate the expression and return an integer that represents the value of the expression.

Valid operators are `+`, `-`, `*`, and `/`. Each operand may be an integer or another expression. Division between two integers should truncate toward zero.

It is guaranteed that the given RPN expression is always valid and will always result in a single number.

## Examples

**Example 1:**
```
Input: tokens = ["2", "1", "+", "3", "*"]
Output: 9
Explanation: ((2 + 1) * 3) = 9
```

**Example 2:**
```
Input: tokens = ["4", "13", "5", "/", "+"]
Output: 6
Explanation: (4 + (13 / 5)) = 6
```

**Example 3:**
```
Input: tokens = ["10", "6", "9", "3", "+", "-11", "*", "/", "*", "17", "+", "5", "+"]
Output: 22
Explanation: ((10 * (6 / ((9 + 3) * -11))) + 17 + 5) = 22
```

## Constraints

- `1 <= tokens.len() <= 10^4`
- Each token is either an operator (`+`, `-`, `*`, `/`) or an integer in the range `[-200, 200]`.

## Hint

Use a stack. When you encounter a number, push it. When you encounter an operator, pop two numbers, apply the operator, and push the result.
