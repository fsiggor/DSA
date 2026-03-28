# Fizz Buzz

## Difficulty: Easy

## Description

Given an integer `n`, return a string array where:
- `answer[i] == "FizzBuzz"` if `i + 1` is divisible by 3 **and** 5.
- `answer[i] == "Fizz"` if `i + 1` is divisible by 3.
- `answer[i] == "Buzz"` if `i + 1` is divisible by 5.
- `answer[i] == i + 1` (as a string) in all other cases.

## Examples

**Example 1:**
```
Input: n = 3
Output: ["1", "2", "Fizz"]
```

**Example 2:**
```
Input: n = 5
Output: ["1", "2", "Fizz", "4", "Buzz"]
```

**Example 3:**
```
Input: n = 15
Output: ["1", "2", "Fizz", "4", "Buzz", "Fizz", "7", "8", "Fizz", "Buzz", "11", "Fizz", "13", "14", "FizzBuzz"]
```

## Constraints

- `1 <= n <= 10^4`

## Hint

Use the modulo operator `%` to check divisibility.
