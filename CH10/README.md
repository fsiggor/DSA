# Intersection of Two Lists

## Difficulty: Easy

## Description

Given two vectors of integers, find the first common element between them. This simulates finding the intersection node of two linked lists.

Return the value of the first element that appears in both vectors. If there is no common element, return `-1`.

## Examples

**Example 1:**
```
Input: list1 = [4, 1, 8, 4, 5], list2 = [5, 6, 1, 8, 4, 5]
Output: 1
Explanation: 1 is the first element in list1 that also appears in list2.
```

**Example 2:**
```
Input: list1 = [1, 9, 1, 2, 4], list2 = [3, 2, 4]
Output: 2
Explanation: 2 is the first element in list1 that also appears in list2.
```

**Example 3:**
```
Input: list1 = [2, 6, 4], list2 = [1, 5]
Output: -1
Explanation: The two lists have no common elements.
```

## Constraints

- `0 <= list1.len(), list2.len() <= 10^4`
- `-10^5 <= list1[i], list2[i] <= 10^5`

## Hint

Use a HashSet to store all elements of one list, then iterate through the other list to find the first match.
