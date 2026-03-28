# Linked List Cycle

## Difficulty: Easy

## Description

Given a list of values and a position index, determine if the linked list would contain a cycle. A cycle exists when a node's next pointer connects back to a previously visited node.

In this simplified version, you receive a vector of values and an integer `cycle_pos` indicating the index where the tail connects back to (-1 means no cycle). Return `true` if there is a cycle, `false` otherwise.

## Examples

**Example 1:**
```
Input: nums = [3, 2, 0, -4], cycle_pos = 1
Output: true
Explanation: The tail connects to the node at index 1, forming a cycle.
```

**Example 2:**
```
Input: nums = [1, 2], cycle_pos = 0
Output: true
Explanation: The tail connects to the node at index 0, forming a cycle.
```

**Example 3:**
```
Input: nums = [1], cycle_pos = -1
Output: false
Explanation: There is no cycle in the list.
```

## Constraints

- `0 <= nums.len() <= 10^4`
- `-10^5 <= nums[i] <= 10^5`
- `cycle_pos` is `-1` or a valid index in `nums`

## Hint

Consider using Floyd's Tortoise and Hare algorithm (two pointers moving at different speeds). In this simplified version, check whether `cycle_pos` is a valid index.
