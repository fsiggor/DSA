# Best Time to Buy and Sell Stock

## Difficulty: Easy

## Description

You are given an array `prices` where `prices[i]` is the price of a given stock on the `i`-th day.

You want to maximize your profit by choosing a **single day** to buy and a **different day in the future** to sell.

Return the **maximum profit** you can achieve. If no profit is possible, return `0`.

## Examples

**Example 1:**
```
Input: prices = [7, 1, 5, 3, 6, 4]
Output: 5
Explanation: Buy on day 2 (price = 1) and sell on day 5 (price = 6), profit = 6 - 1 = 5.
```

**Example 2:**
```
Input: prices = [7, 6, 4, 3, 1]
Output: 0
Explanation: No transaction yields a positive profit, so the maximum profit is 0.
```

## Constraints

- `1 <= prices.len() <= 10^5`
- `0 <= prices[i] <= 10^4`

## Hint

Track the minimum price seen so far and compute the profit for each day. The maximum profit is the largest difference found.
