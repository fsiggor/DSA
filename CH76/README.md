# Course Schedule

## Difficulty: Medium

## Description

There are a total of `num_courses` courses you have to take, labeled from `0` to `num_courses - 1`. You are given an array `prerequisites` where `prerequisites[i] = [a, b]` indicates that you must take course `b` before course `a`.

Return `true` if you can finish all courses. Otherwise, return `false`.

## Examples

**Example 1:**
```
Input: num_courses = 2, prerequisites = [[1, 0]]
Output: true
Explanation: You take course 0 first, then course 1.
```

**Example 2:**
```
Input: num_courses = 2, prerequisites = [[1, 0], [0, 1]]
Output: false
Explanation: There is a cycle between courses 0 and 1.
```

## Constraints

- `1 <= num_courses <= 2000`
- `0 <= prerequisites.len() <= 5000`
- `prerequisites[i].len() == 2`
- `0 <= a, b < num_courses`
- All prerequisite pairs are unique.

## Hint

This is a cycle detection problem in a directed graph. Use topological sort (BFS with in-degree counting) or DFS with visited states.
