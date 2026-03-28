# Zigzag Conversion

## Difficulty: Medium

## Description

The string `"PAYPALISHIRING"` is written in a zigzag pattern on a given number of rows like this:

```
P   A   H   N
A P L S I I G
Y   I   R
```

And then read line by line: `"PAHNAPLSIIGYIR"`.

Write a function that takes a string and a number of rows and produces the zigzag output string read line by line.

## Examples

**Example 1:**
```
Input: s = "PAYPALISHIRING", num_rows = 3
Output: "PAHNAPLSIIGYIR"
```

**Example 2:**
```
Input: s = "PAYPALISHIRING", num_rows = 4
Output: "PINALSIGYAHRPI"
Explanation:
P     I    N
A   L S  I G
Y A   H R
P     I
```

**Example 3:**
```
Input: s = "A", num_rows = 1
Output: "A"
```

## Constraints

- `1 <= s.len() <= 1000`
- `s` consists of English letters (lower-case and upper-case), `','` and `'.'`.
- `1 <= num_rows <= 1000`

## Hint

Simulate the zigzag by maintaining a vector of strings (one per row). Iterate through the characters, adding each to the current row, and change direction when you reach the top or bottom row.
