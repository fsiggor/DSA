# Valid Anagram

## Difficulty: Easy

## Description

Given two strings `s` and `t`, return `true` if `t` is an **anagram** of `s`, and `false` otherwise.

An anagram is a word formed by rearranging the letters of another word, using all the original letters exactly once.

## Examples

**Example 1:**
```
Input: s = "anagram", t = "nagaram"
Output: true
```

**Example 2:**
```
Input: s = "rat", t = "car"
Output: false
```

## Constraints

- `1 <= s.len(), t.len() <= 5 * 10^4`
- `s` and `t` consist of only lowercase English letters.

## Hint

Count the frequency of each character in both strings and compare. You can use a 26-element array or a HashMap.
