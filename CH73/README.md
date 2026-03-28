# Coin Change

## Difficulty: Medium

## Description

You are given an integer array `coins` representing coins of different denominations and an integer `amount` representing a total amount of money.

Return the fewest number of coins that you need to make up that amount. If that amount of money cannot be made up by any combination of the coins, return `-1`.

You may assume that you have an infinite number of each kind of coin.

## Examples

**Example 1:**
```
Input: coins = [1, 5, 11], amount = 15
Output: 3
Explanation: 15 = 5 + 5 + 5
```

**Example 2:**
```
Input: coins = [2], amount = 3
Output: -1
Explanation: No combination of coins can make 3.
```

**Example 3:**
```
Input: coins = [1], amount = 0
Output: 0
```

## Constraints

- `1 <= coins.len() <= 12`
- `1 <= coins[i] <= 2^31 - 1`
- `0 <= amount <= 10^4`

## Hint

Use dynamic programming with a 1D array where `dp[i]` represents the minimum number of coins needed to make amount `i`. Build up from 0 to the target amount.
