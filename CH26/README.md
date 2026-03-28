# Word Search

## Difficulty: Medium

## Description

Given an `m x n` grid of characters `board` and a string `word`, return `true` if `word` exists in the grid.

The word can be constructed from letters of sequentially adjacent cells, where adjacent cells are horizontally or vertically neighboring. The same cell may not be used more than once.

## Examples

**Example 1:**
```
Input: board = [["A","B","C","E"],["S","F","C","S"],["A","D","E","E"]], word = "ABCCED"
Output: true
```

**Example 2:**
```
Input: board = [["A","B","C","E"],["S","F","C","S"],["A","D","E","E"]], word = "SEE"
Output: true
```

**Example 3:**
```
Input: board = [["A","B","C","E"],["S","F","C","S"],["A","D","E","E"]], word = "ABCB"
Output: false
```

## Constraints

- `m == board.len()`
- `n == board[i].len()`
- `1 <= m, n <= 6`
- `1 <= word.len() <= 15`
- `board` and `word` consist of only lowercase and uppercase English letters.

## Hint

Use backtracking with DFS. For each cell, try to match the word starting from that cell. Mark cells as visited during exploration and unmark them when backtracking.
