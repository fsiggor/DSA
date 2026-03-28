# Min Stack

## Difficulty: Medium

## Description

Design a stack that supports push, pop, top, and retrieving the minimum element in constant time.

Implement the `MinStack` struct with the following methods:

- `MinStack::new()` - Initializes the stack object.
- `push(val: i32)` - Pushes the element `val` onto the stack.
- `pop()` - Removes the element on the top of the stack.
- `top() -> i32` - Gets the top element of the stack.
- `get_min() -> i32` - Retrieves the minimum element in the stack.

Each method must operate in O(1) time complexity.

## Examples

**Example 1:**
```
Input:
  MinStack::new()
  push(-2)
  push(0)
  push(-3)
  get_min() -> -3
  pop()
  top() -> 0
  get_min() -> -2
```

## Constraints

- `-2^31 <= val <= 2^31 - 1`
- Methods `pop`, `top`, and `get_min` will always be called on non-empty stacks.
- At most `3 * 10^4` calls will be made to `push`, `pop`, `top`, and `get_min`.

## Hint

Use two stacks: one for the actual values and one to track the current minimum at each level.
