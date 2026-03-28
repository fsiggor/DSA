# Container With Most Water

## Difficulty: Medium

## Description

You are given an integer array `height` of length `n`. There are `n` vertical lines drawn such that the two endpoints of the `i`-th line are `(i, 0)` and `(i, height[i])`.

Find two lines that together with the x-axis form a container, such that the container holds the most water.

Return the maximum amount of water a container can store.

**Note:** You may not slant the container.

## Examples

**Example 1:**
```
Input: height = [1, 8, 6, 2, 5, 4, 8, 3, 7]
Output: 49
Explanation: Lines at index 1 (height 8) and index 8 (height 7) form a container of area min(8, 7) * (8 - 1) = 49.
```

**Example 2:**
```
Input: height = [1, 1]
Output: 1
```

**Example 3:**
```
Input: height = [4, 3, 2, 1, 4]
Output: 16
Explanation: Lines at index 0 (height 4) and index 4 (height 4) form a container of area min(4, 4) * (4 - 0) = 16.
```

## Constraints

- `2 <= height.len() <= 10^5`
- `0 <= height[i] <= 10^4`

## Hint

Use two pointers starting from both ends of the array. Move the pointer pointing to the shorter line inward, since moving the taller one can never increase the area.
