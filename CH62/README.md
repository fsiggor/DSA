# Sort List

## Difficulty: Medium

## Description

Given a vector of integers, sort it in ascending order using merge sort.

Merge sort is a divide-and-conquer algorithm that splits the list in half, recursively sorts each half, and then merges the sorted halves.

## Examples

**Example 1:**
```
Input: list = [4, 2, 1, 3]
Output: [1, 2, 3, 4]
```

**Example 2:**
```
Input: list = [-1, 5, 3, 4, 0]
Output: [-1, 0, 3, 4, 5]
```

**Example 3:**
```
Input: list = []
Output: []
```

## Constraints

- `0 <= list.len() <= 5 * 10^4`
- `-10^5 <= list[i] <= 10^5`

## Hint

Split the vector into two halves, recursively sort each half, then merge the two sorted halves by comparing elements one at a time.
