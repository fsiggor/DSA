# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## What This Is

90 classic technical interview problems (LeetCode-style) implemented in Rust. Each `CH##/` folder contains a `README.md` (problem statement) and `solution.rs` (empty function signature + unit tests). Numbering is shuffled — not ordered by difficulty.

## Build & Test

```bash
# Test a specific challenge (the primary workflow)
rustc --edition 2021 --test CH42/solution.rs -o test_bin && ./test_bin

# Run a single test by name
rustc --edition 2021 --test CH42/solution.rs -o test_bin && ./test_bin test_example_1

# Pick a random challenge
ls -d CH*/ | shuf | head -1
```

No Cargo project — each `solution.rs` is a standalone file compiled directly with `rustc --edition 2021 --test`.

## Structure

- `CH01/`–`CH90/` — Each challenge has `README.md` (problem, examples, constraints, hint) and `solution.rs` (function stub with `todo!()` + `#[cfg(test)]` unit tests)
- `.challenge_map.json` — Maps `CH##` codes to canonical problem names
- Solutions use `Vec`-based representations (not linked list pointers, not tree node pointers) to keep Rust ergonomics simple

## Working on Solutions

- Implement the function body in `solution.rs`, replacing the `todo!()` macro
- Do not modify test cases or function signatures
- All solutions are pure functions — no external crates, no `main()`, no I/O
- Categories: Arrays/Two Pointers, Strings, Linked Lists, Stacks/Queues, Binary Search, Trees, DP, Graphs, Backtracking, Sorting/Searching, Bit Manipulation, Math
