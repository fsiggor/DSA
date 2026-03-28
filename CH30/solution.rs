pub fn is_power_of_two(n: i32) -> bool {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        assert!(is_power_of_two(1));
    }

    #[test]
    fn test_two() {
        assert!(is_power_of_two(2));
    }

    #[test]
    fn test_sixteen() {
        assert!(is_power_of_two(16));
    }

    #[test]
    fn test_three() {
        assert!(!is_power_of_two(3));
    }

    #[test]
    fn test_zero() {
        assert!(!is_power_of_two(0));
    }

    #[test]
    fn test_negative() {
        assert!(!is_power_of_two(-1));
    }

    #[test]
    fn test_negative_power() {
        assert!(!is_power_of_two(-16));
    }

    #[test]
    fn test_large_power() {
        assert!(is_power_of_two(1073741824)); // 2^30
    }

    #[test]
    fn test_not_power_large() {
        assert!(!is_power_of_two(1073741823)); // 2^30 - 1
    }

    #[test]
    fn test_four() {
        assert!(is_power_of_two(4));
    }

    #[test]
    fn test_six() {
        assert!(!is_power_of_two(6));
    }

    #[test]
    fn test_min_value() {
        assert!(!is_power_of_two(i32::MIN));
    }
}
