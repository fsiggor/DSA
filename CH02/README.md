# Count and Say

## Difficulty: Medium

## Description

The **count-and-say** sequence is a sequence of digit strings defined by the recursive formula:

- `count_and_say(1) = "1"`
- `count_and_say(n)` is the run-length encoding of `count_and_say(n - 1)`.

Run-length encoding (RLE) is a string compression method that works by replacing consecutive identical characters with the character followed by its count. For example, to compress `"3322251"` we replace `"33"` with `"23"`, replace `"222"` with `"32"`, replace `"5"` with `"15"`, and replace `"1"` with `"11"`. Thus the compressed string becomes `"23321511"`.

Given a positive integer `n`, return the `n`th element of the count-and-say sequence.

## Examples

**Example 1:**
```
Input: n = 1
Output: "1"
```

**Example 2:**
```
Input: n = 4
Output: "1211"
Explanation:
count_and_say(1) = "1"
count_and_say(2) = "11"   (one 1 -> "11")
count_and_say(3) = "21"   (two 1s -> "21")
count_and_say(4) = "1211" (one 2, then one 1 -> "1211")
```

## Constraints

- `1 <= n <= 30`

## Hint

Build the sequence iteratively. For each term, iterate through the previous term and count consecutive identical digits to generate the next term.
