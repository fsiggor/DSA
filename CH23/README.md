# Clone Graph

**Difficulty:** Medium

**Category:** Graphs

## Description

Given a graph represented as an adjacency list, return a deep copy (clone) of the graph.

Each node in the graph is identified by its index (0-based). The adjacency list `adj_list[i]` contains the indices of all neighbors of node `i`.

Return a new adjacency list that is a deep copy of the input.

## Examples

**Example 1:**

```
Input: adj_list = [[1,2],[0,2],[0,1]]
Output: [[1,2],[0,2],[0,1]]
Explanation: A triangle graph with 3 nodes. The output is a deep copy.
```

**Example 2:**

```
Input: adj_list = [[]]
Output: [[]]
Explanation: A single node with no neighbors.
```

**Example 3:**

```
Input: adj_list = []
Output: []
Explanation: An empty graph.
```

## Constraints

- `0 <= adj_list.length <= 100`
- `0 <= adj_list[i][j] < adj_list.length`
- There are no self-loops or duplicate edges.

## Hint

Since the graph is represented as an adjacency list using indices, a deep copy means creating entirely new vectors. You can use DFS or BFS to traverse and clone, or simply clone each adjacency list entry.
