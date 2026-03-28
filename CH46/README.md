# Valid Palindrome

## Difficulty: Easy

## Description

Given a string `s`, determine if it is a palindrome, considering only alphanumeric characters and ignoring case differences.

## Examples

**Example 1:**
```
Input: s = "A man, a plan, a canal: Panama"
Output: true
Explanation: "amanaplanacanalpanama" is a palindrome.
```

**Example 2:**
```
Input: s = "race a car"
Output: false
Explanation: "raceacar" is not a palindrome.
```

**Example 3:**
```
Input: s = " "
Output: true
Explanation: After removing non-alphanumeric characters, s is an empty string "". An empty string is a palindrome by definition.
```

## Constraints

- `1 <= s.len() <= 2 * 10^5`
- `s` consists only of printable ASCII characters.

## Hint

Use two pointers, one at the start and one at the end of the string, skipping non-alphanumeric characters.
