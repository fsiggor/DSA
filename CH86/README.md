# Palindrome Number

## Difficulty: Easy

## Description

Given an integer `x`, return `true` if `x` is a palindrome number.

A number is a palindrome when it reads the same backward as forward. For example, `121` is a palindrome while `123` is not.

## Examples

**Example 1:**
```
Input: x = 121
Output: true
```

**Example 2:**
```
Input: x = -121
Output: false
Explanation: From left to right: -121. From right to left: 121-. Therefore it is not a palindrome.
```

**Example 3:**
```
Input: x = 10
Output: false
Explanation: Reads 01 from right to left. Therefore it is not a palindrome.
```

## Constraints

- `-2^31 <= x <= 2^31 - 1`

## Hint

Try solving it without converting to a string. You can reverse half of the number and compare it with the other half.
