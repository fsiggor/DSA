# Longest Common Prefix

## Difficulty: Easy

## Description

Write a function to find the **longest common prefix** string amongst an array of strings.

If there is no common prefix, return an empty string `""`.

## Examples

**Example 1:**
```
Input: strs = ["flower", "flow", "flight"]
Output: "fl"
```

**Example 2:**
```
Input: strs = ["dog", "racecar", "car"]
Output: ""
Explanation: There is no common prefix among the input strings.
```

## Constraints

- `1 <= strs.len() <= 200`
- `0 <= strs[i].len() <= 200`
- `strs[i]` consists of only lowercase English letters.

## Hint

Compare character by character, using the first string as a reference. Stop when you find a mismatch or when any string ends.
