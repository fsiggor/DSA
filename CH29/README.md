# Word Ladder

## Difficulty: Hard

## Description

A **transformation sequence** from word `begin_word` to word `end_word` using a dictionary `word_list` is a sequence of words such that:

- The first word is `begin_word`.
- The last word is `end_word`.
- Each adjacent pair of words differs by exactly one letter.
- Every word in the sequence (except `begin_word`) is in `word_list`.

Given `begin_word`, `end_word`, and `word_list`, return the **number of words** in the shortest transformation sequence, or `0` if no such sequence exists.

## Examples

**Example 1:**
```
Input: begin_word = "hit", end_word = "cog", word_list = ["hot","dot","dog","lot","log","cog"]
Output: 5
Explanation: "hit" -> "hot" -> "dot" -> "dog" -> "cog"
```

**Example 2:**
```
Input: begin_word = "hit", end_word = "cog", word_list = ["hot","dot","dog","lot","log"]
Output: 0
Explanation: end_word "cog" is not in word_list.
```

## Constraints

- `1 <= begin_word.len() <= 10`
- `end_word.len() == begin_word.len()`
- `1 <= word_list.len() <= 5000`
- All words have the same length.
- All words consist of lowercase English letters.
- `begin_word != end_word`

## Hint

Use BFS. Each word is a node, and two words are connected if they differ by exactly one letter. BFS guarantees the shortest path.
