# Longest Palindromic Substring

## Difficulty: Medium

## Description

Given a string `s`, return the **longest palindromic substring** in `s`.

A **palindrome** is a string that reads the same forward and backward.

## Examples

**Example 1:**
```
Input: s = "babad"
Output: "bab"
Explanation: "aba" is also a valid answer.
```

**Example 2:**
```
Input: s = "cbbd"
Output: "bb"
```

**Example 3:**
```
Input: s = "a"
Output: "a"
```

## Constraints

- `1 <= s.len() <= 1000`
- `s` consists of only lowercase English letters.

## Hint

Try expanding around each center. A palindrome can be centered at a single character (odd length) or between two characters (even length). Expand outward while the characters match.
