# Queue Using Stacks

## Difficulty: Easy

## Description

Implement a first-in-first-out (FIFO) queue using only two stacks (represented as `Vec<i32>`). The implemented queue should support all the functions of a normal queue: `push`, `pop`, `peek`, and `empty`.

Implement the `MyQueue` struct with the following methods:

- `MyQueue::new()` - Initializes the queue object.
- `push(x: i32)` - Pushes element `x` to the back of the queue.
- `pop() -> i32` - Removes the element from the front of the queue and returns it.
- `peek() -> i32` - Returns the element at the front of the queue without removing it.
- `empty() -> bool` - Returns `true` if the queue is empty, `false` otherwise.

## Examples

**Example 1:**
```
Input:
  MyQueue::new()
  push(1)
  push(2)
  peek() -> 1
  pop() -> 1
  empty() -> false
```

## Constraints

- `1 <= x <= 9`
- At most `100` calls will be made to `push`, `pop`, `peek`, and `empty`.
- All calls to `pop` and `peek` are valid (the queue is non-empty).

## Hint

Use two stacks: one for pushing and one for popping. When the pop stack is empty, transfer all elements from the push stack to reverse their order.
