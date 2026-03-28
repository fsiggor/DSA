# 72. Graph Valid Tree

**Difficulty:** Medium

**Category:** Graphs / Union-Find

## Description

You have a graph of `n` nodes labeled from `0` to `n - 1`. You are given an integer `n` and a list of `edges` where `edges[i] = [ai, bi]` indicates that there is an undirected edge between nodes `ai` and `bi` in the graph.

Return `true` if the edges of the given graph make up a valid tree, and `false` otherwise.

A valid tree must satisfy:
- It is fully connected (all nodes are reachable from any node).
- It contains no cycles.
- It has exactly `n - 1` edges.

## Examples

**Example 1:**

```
Input: n = 5, edges = [[0,1],[0,2],[0,3],[1,4]]
Output: true
```

**Example 2:**

```
Input: n = 5, edges = [[0,1],[1,2],[2,3],[1,3],[1,4]]
Output: false
```

**Example 3:**

```
Input: n = 1, edges = []
Output: true
```

## Constraints

- `1 <= n <= 2000`
- `0 <= edges.length <= 5000`
- `edges[i].length == 2`
- `0 <= ai, bi < n`
- `ai != bi`
- There are no self-loops or repeated edges.

## Hint

A graph is a valid tree if and only if it has exactly `n - 1` edges and is fully connected. Use Union-Find to detect cycles: if two nodes are already in the same set when processing an edge, a cycle exists.
