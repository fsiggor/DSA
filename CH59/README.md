# String to Integer (atoi)

## Difficulty: Medium

## Description

Implement the `my_atoi` function, which converts a string to a 32-bit signed integer.

The algorithm is as follows:

1. Read in and ignore any leading whitespace.
2. Check if the next character is `'-'` or `'+'`. Read this character in if it is either. This determines if the final result is negative or positive. Assume the result is positive if neither is present.
3. Read in the next characters until the next non-digit character or the end of the input is reached. The rest of the string is ignored.
4. Convert these digits into an integer. If no digits were read, then the integer is `0`.
5. If the integer is out of the 32-bit signed integer range `[-2^31, 2^31 - 1]`, clamp the integer so that it remains in the range. Specifically, integers less than `-2^31` should be clamped to `-2^31`, and integers greater than `2^31 - 1` should be clamped to `2^31 - 1`.
6. Return the integer as the final result.

## Examples

**Example 1:**
```
Input: s = "42"
Output: 42
```

**Example 2:**
```
Input: s = "   -42"
Output: -42
Explanation: Leading whitespace is skipped, then '-' sign is read.
```

**Example 3:**
```
Input: s = "4193 with words"
Output: 4193
Explanation: Conversion stops at '1' because the next character is not a digit.
```

**Example 4:**
```
Input: s = "words and 987"
Output: 0
Explanation: The first non-whitespace character is 'w', which is not a digit or '+'/'-'. No valid conversion can be performed.
```

## Constraints

- `0 <= s.len() <= 200`
- `s` consists of English letters (lower-case and upper-case), digits (0-9), `' '`, `'+'`, `'-'`, and `'.'`.

## Hint

Process the string character by character. Handle whitespace first, then the optional sign, then digits. Be careful with overflow: check before multiplying by 10 whether the result would exceed `i32::MAX` or `i32::MIN`.
