# Decode Ways

**Difficulty:** Medium

**Category:** Dynamic Programming

## Description

A message containing letters from `A-Z` can be encoded into numbers using the following mapping:

```
'A' -> "1"
'B' -> "2"
...
'Z' -> "26"
```

To decode an encoded message, all the digits must be grouped then mapped back into letters using the reverse of the mapping above. There may be multiple ways to do so.

Given a string `s` containing only digits, return the number of ways to decode it.

## Examples

**Example 1:**

```
Input: s = "12"
Output: 2
Explanation: "12" could be decoded as "AB" (1 2) or "L" (12).
```

**Example 2:**

```
Input: s = "226"
Output: 3
Explanation: "226" could be decoded as "BZ" (2 26), "VF" (22 6), or "BBF" (2 2 6).
```

**Example 3:**

```
Input: s = "06"
Output: 0
Explanation: "06" cannot be mapped to "F" because of the leading zero. "6" is different from "06".
```

## Constraints

- `1 <= s.length <= 100`
- `s` contains only digits and may contain leading zeros.

## Hint

Use dynamic programming where `dp[i]` represents the number of ways to decode the substring `s[0..i]`. Check if the current digit and the two-digit number ending at the current position are valid decodings.
