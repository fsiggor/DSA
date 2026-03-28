# Technical Interview Challenges in Rust

90 classic technical interview problems from Big Tech companies, organized for practice in Rust.
The numbering is shuffled — CH01 is not the easiest and CH90 is not the hardest.

## How to use

Each folder contains:
- `README.md` — Problem statement
- `solution.rs` — Function signature (empty) and unit tests

```bash
# Pick a random challenge
ls -d CH*/ | shuf | head -1

# Read the problem
cat CH42/README.md

# Test your solution
rustc --edition 2021 --test CH42/solution.rs -o test_bin && ./test_bin
```

## Problems

### Arrays / Two Pointers
| Code   | Problem                              | Difficulty |
| ------ | ------------------------------------ | ---------- |
| `CH15` | Best Time To Buy And Sell Stock      | Easy       |
| `CH31` | Contains Duplicate                   | Easy       |
| `CH80` | Merge Sorted Arrays                  | Easy       |
| `CH17` | Move Zeroes                          | Easy       |
| `CH37` | Remove Duplicates From Sorted Array  | Easy       |
| `CH35` | Two Sum                              | Easy       |
| `CH07` | Container With Most Water            | Medium     |
| `CH67` | Next Permutation                     | Medium     |
| `CH42` | Product Of Array Except Self         | Medium     |
| `CH78` | Rotate Array                         | Medium     |
| `CH45` | Sort Colors                          | Medium     |
| `CH12` | Three Sum                            | Medium     |
| `CH40` | First Missing Positive               | Hard       |
| `CH55` | Trapping Rain Water                  | Hard       |

### Strings
| Code   | Problem                              | Difficulty |
| ------ | ------------------------------------ | ---------- |
| `CH43` | Implement strStr                     | Easy       |
| `CH11` | Longest Common Prefix                | Easy       |
| `CH21` | Valid Anagram                        | Easy       |
| `CH46` | Valid Palindrome                     | Easy       |
| `CH02` | Count And Say                        | Medium     |
| `CH85` | Group Anagrams                       | Medium     |
| `CH84` | Longest Palindromic Substring        | Medium     |
| `CH68` | Longest Substring Without Repeating  | Medium     |
| `CH59` | String To Integer (atoi)             | Medium     |
| `CH64` | Zigzag Conversion                    | Medium     |
| `CH61` | Minimum Window Substring             | Hard       |

### Linked Lists
| Code   | Problem                              | Difficulty |
| ------ | ------------------------------------ | ---------- |
| `CH10` | Intersection Of Two Lists            | Easy       |
| `CH82` | Linked List Cycle                    | Easy       |
| `CH75` | Merge Two Sorted Lists               | Easy       |
| `CH56` | Reverse Linked List                  | Easy       |
| `CH90` | Add Two Numbers                      | Medium     |
| `CH01` | Remove Nth Node From End             | Medium     |
| `CH62` | Sort List                            | Medium     |

### Stacks / Queues
| Code   | Problem                              | Difficulty |
| ------ | ------------------------------------ | ---------- |
| `CH54` | Queue Using Stacks                   | Easy       |
| `CH63` | Valid Parentheses                    | Easy       |
| `CH27` | Daily Temperatures                   | Medium     |
| `CH36` | Evaluate Reverse Polish Notation     | Medium     |
| `CH89` | Min Stack                            | Medium     |
| `CH83` | Largest Rectangle In Histogram       | Hard       |

### Binary Search
| Code   | Problem                              | Difficulty |
| ------ | ------------------------------------ | ---------- |
| `CH41` | Binary Search                        | Easy       |
| `CH60` | Find Min In Rotated Sorted Array     | Medium     |
| `CH08` | Find Peak Element                    | Medium     |
| `CH03` | Search In Rotated Sorted Array       | Medium     |

### Trees
| Code   | Problem                              | Difficulty |
| ------ | ------------------------------------ | ---------- |
| `CH32` | Invert Binary Tree                   | Easy       |
| `CH53` | Max Depth Binary Tree                | Easy       |
| `CH50` | Symmetric Tree                       | Easy       |
| `CH33` | Binary Tree Level Order              | Medium     |
| `CH25` | Binary Tree Right Side View          | Medium     |
| `CH39` | Construct Tree From Preorder Inorder | Medium     |
| `CH58` | Kth Smallest In BST                  | Medium     |
| `CH72` | Lowest Common Ancestor BST           | Medium     |
| `CH14` | Validate BST                         | Medium     |
| `CH66` | Serialize Deserialize Binary Tree    | Hard       |

### Dynamic Programming
| Code   | Problem                              | Difficulty |
| ------ | ------------------------------------ | ---------- |
| `CH79` | Climbing Stairs                      | Easy       |
| `CH73` | Coin Change                          | Medium     |
| `CH13` | Decode Ways                          | Medium     |
| `CH16` | Edit Distance                        | Medium     |
| `CH51` | House Robber                         | Medium     |
| `CH47` | Jump Game                            | Medium     |
| `CH44` | Longest Common Subsequence           | Medium     |
| `CH69` | Longest Increasing Subsequence       | Medium     |
| `CH06` | Maximum Product Subarray             | Medium     |
| `CH20` | Maximum Subarray                     | Medium     |
| `CH74` | Unique Paths                         | Medium     |
| `CH48` | Word Break                           | Medium     |

### Graphs
| Code   | Problem                              | Difficulty |
| ------ | ------------------------------------ | ---------- |
| `CH23` | Clone Graph                          | Medium     |
| `CH76` | Course Schedule                      | Medium     |
| `CH18` | Graph Valid Tree                     | Medium     |
| `CH52` | Number Of Islands                    | Medium     |
| `CH65` | Pacific Atlantic Water Flow          | Medium     |
| `CH29` | Word Ladder                          | Hard       |

### Backtracking
| Code   | Problem                              | Difficulty |
| ------ | ------------------------------------ | ---------- |
| `CH57` | Combination Sum                      | Medium     |
| `CH38` | Generate Parentheses                 | Medium     |
| `CH34` | Letter Combinations Of Phone         | Medium     |
| `CH70` | Permutations                         | Medium     |
| `CH49` | Subsets                              | Medium     |
| `CH26` | Word Search                          | Medium     |
| `CH88` | N Queens                             | Hard       |

### Sorting / Searching
| Code   | Problem                              | Difficulty |
| ------ | ------------------------------------ | ---------- |
| `CH28` | Kth Largest Element                  | Medium     |
| `CH24` | Merge Intervals                      | Medium     |
| `CH71` | Top K Frequent Elements              | Medium     |

### Bit Manipulation
| Code   | Problem                              | Difficulty |
| ------ | ------------------------------------ | ---------- |
| `CH81` | Counting Bits                        | Easy       |
| `CH22` | Missing Number                       | Easy       |
| `CH30` | Power Of Two                         | Easy       |
| `CH05` | Single Number                        | Easy       |

### Math
| Code   | Problem                              | Difficulty |
| ------ | ------------------------------------ | ---------- |
| `CH87` | Excel Sheet Column Number            | Easy       |
| `CH77` | Fizz Buzz                            | Easy       |
| `CH09` | Happy Number                         | Easy       |
| `CH86` | Palindrome Number                    | Easy       |
| `CH04` | Roman To Integer                     | Easy       |
| `CH19` | Reverse Integer                      | Medium     |

## Summary

| Difficulty | Count |
| ---------- | ----- |
| Easy       | 30    |
| Medium     | 52    |
| Hard       | 8     |
| **Total**  | **90**|

## Source

Problems based on classic questions widely available on public platforms such as LeetCode, HackerRank, and open-source interview preparation repositories.
