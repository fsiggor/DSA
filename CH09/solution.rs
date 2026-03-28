pub fn is_happy(n: i32) -> bool {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nineteen() {
        assert!(is_happy(19));
    }

    #[test]
    fn test_two() {
        assert!(!is_happy(2));
    }

    #[test]
    fn test_one() {
        assert!(is_happy(1));
    }

    #[test]
    fn test_seven() {
        assert!(is_happy(7));
    }

    #[test]
    fn test_four() {
        assert!(!is_happy(4));
    }

    #[test]
    fn test_ten() {
        assert!(is_happy(10));
    }

    #[test]
    fn test_hundred() {
        assert!(is_happy(100));
    }

    #[test]
    fn test_three() {
        assert!(!is_happy(3));
    }

    #[test]
    fn test_twenty_three() {
        assert!(is_happy(23));
    }

    #[test]
    fn test_large_unhappy() {
        assert!(!is_happy(116));
    }
}
