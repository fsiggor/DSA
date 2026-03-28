# Implement strStr()

## Difficulty: Easy

## Description

Given two strings `haystack` and `needle`, return the index of the first occurrence of `needle` in `haystack`, or `-1` if `needle` is not part of `haystack`.

## Examples

**Example 1:**
```
Input: haystack = "sadbutsad", needle = "sad"
Output: 0
Explanation: "sad" occurs at index 0 and 6. The first occurrence is at index 0.
```

**Example 2:**
```
Input: haystack = "leetcode", needle = "leeto"
Output: -1
Explanation: "leeto" did not occur in "leetcode", so we return -1.
```

**Example 3:**
```
Input: haystack = "hello", needle = "ll"
Output: 2
```

## Constraints

- `1 <= haystack.len(), needle.len() <= 10^4`
- `haystack` and `needle` consist of only lowercase English characters.

## Hint

A simple approach is to check every possible starting position in `haystack` and compare the substring. For a more efficient solution, consider the KMP (Knuth-Morris-Pratt) algorithm.
