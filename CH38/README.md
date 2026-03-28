# Generate Parentheses

## Difficulty: Medium

## Description

Given `n` pairs of parentheses, write a function to generate all combinations of well-formed parentheses.

## Examples

**Example 1:**
```
Input: n = 3
Output: ["((()))","(()())","(())()","()(())","()()()"]
```

**Example 2:**
```
Input: n = 1
Output: ["()"]
```

## Constraints

- `1 <= n <= 8`

## Hint

Use backtracking. At each step, you can add an opening parenthesis if you haven't used all `n`, or a closing parenthesis if the number of closing ones is less than the number of opening ones used so far.
