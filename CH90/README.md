# Add Two Numbers

## Difficulty: Medium

## Description

You are given two non-empty vectors representing two non-negative integers. The digits are stored in **reverse order**, and each element contains a single digit. Add the two numbers and return the sum as a reversed-digit vector.

You may assume the two numbers do not contain any leading zero, except the number 0 itself.

## Examples

**Example 1:**
```
Input: l1 = [2, 4, 3], l2 = [5, 6, 4]
Output: [7, 0, 8]
Explanation: 342 + 465 = 807.
```

**Example 2:**
```
Input: l1 = [0], l2 = [0]
Output: [0]
```

**Example 3:**
```
Input: l1 = [9, 9, 9, 9, 9, 9, 9], l2 = [9, 9, 9, 9]
Output: [8, 9, 9, 9, 0, 0, 0, 1]
Explanation: 9999999 + 9999 = 10009998.
```

## Constraints

- `1 <= l1.len(), l2.len() <= 100`
- `0 <= l1[i], l2[i] <= 9`
- It is guaranteed that the list represents a number that does not have leading zeros.

## Hint

Iterate through both vectors simultaneously, summing corresponding digits along with any carry. Remember to handle the final carry if it is non-zero.
