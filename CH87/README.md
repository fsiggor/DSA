# 90. Excel Sheet Column Number

**Difficulty:** Easy

**Category:** Math

## Description

Given a string `column_title` that represents the column title as it appears in an Excel sheet, return its corresponding column number.

The mapping is as follows:

```
A -> 1
B -> 2
C -> 3
...
Z -> 26
AA -> 27
AB -> 28
...
```

## Examples

**Example 1:**

```
Input: column_title = "A"
Output: 1
```

**Example 2:**

```
Input: column_title = "AB"
Output: 28
```

**Example 3:**

```
Input: column_title = "ZY"
Output: 701
```

## Constraints

- `1 <= column_title.length <= 7`
- `column_title` consists only of uppercase English letters
- `column_title` is in the range `["A", "FXSHRXW"]`

## Hint

Think of the column title as a base-26 number where `A = 1, B = 2, ..., Z = 26`. Process each character from left to right, multiplying the running total by 26 and adding the current character's value.
