# Roman to Integer

## Difficulty: Easy

## Description

Roman numerals are represented by seven different symbols:

| Symbol | Value |
| ------ | ----- |
| I      | 1     |
| V      | 5     |
| X      | 10    |
| L      | 50    |
| C      | 100   |
| D      | 500   |
| M      | 1000  |

When a smaller symbol appears before a larger one, it is subtracted. For example:
- `IV` = 4, `IX` = 9
- `XL` = 40, `XC` = 90
- `CD` = 400, `CM` = 900

Given a string representing a Roman numeral, convert it to an integer.

## Examples

**Example 1:**
```
Input: s = "III"
Output: 3
```

**Example 2:**
```
Input: s = "LVIII"
Output: 58
Explanation: L = 50, V = 5, III = 3.
```

**Example 3:**
```
Input: s = "MCMXCIV"
Output: 1994
Explanation: M = 1000, CM = 900, XC = 90, IV = 4.
```

## Constraints

- `1 <= s.len() <= 15`
- `s` contains only the characters `I, V, X, L, C, D, M`.
- The value is in the range `[1, 3999]`.

## Hint

Traverse the string left to right. If the current value is less than the next, subtract it; otherwise, add it.
