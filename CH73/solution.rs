pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
    // Implement your solution here
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(coin_change(vec![1, 5, 11], 15), 3);
    }

    #[test]
    fn test_impossible() {
        assert_eq!(coin_change(vec![2], 3), -1);
    }

    #[test]
    fn test_zero_amount() {
        assert_eq!(coin_change(vec![1], 0), 0);
    }

    #[test]
    fn test_single_coin_exact() {
        assert_eq!(coin_change(vec![1], 5), 5);
    }

    #[test]
    fn test_multiple_coins() {
        assert_eq!(coin_change(vec![1, 2, 5], 11), 3);
    }

    #[test]
    fn test_greedy_fails() {
        // Greedy would pick 11 + 1 + 1 + 1 + 1 = 5 coins, but optimal is 5 + 5 + 5 = 3 coins
        assert_eq!(coin_change(vec![1, 5, 11], 15), 3);
    }

    #[test]
    fn test_large_amount() {
        assert_eq!(coin_change(vec![1, 5, 10, 25], 100), 4);
    }

    #[test]
    fn test_single_denomination() {
        assert_eq!(coin_change(vec![3], 9), 3);
    }

    #[test]
    fn test_single_denomination_impossible() {
        assert_eq!(coin_change(vec![3], 7), -1);
    }

    #[test]
    fn test_amount_equals_coin() {
        assert_eq!(coin_change(vec![1, 5, 10], 5), 1);
    }
}
