pub fn reverse(x: i32) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_positive() {
        assert_eq!(reverse(123), 321);
    }

    #[test]
    fn test_negative() {
        assert_eq!(reverse(-123), -321);
    }

    #[test]
    fn test_trailing_zero() {
        assert_eq!(reverse(120), 21);
    }

    #[test]
    fn test_zero() {
        assert_eq!(reverse(0), 0);
    }

    #[test]
    fn test_single_digit() {
        assert_eq!(reverse(5), 5);
    }

    #[test]
    fn test_negative_single_digit() {
        assert_eq!(reverse(-8), -8);
    }

    #[test]
    fn test_overflow_positive() {
        assert_eq!(reverse(1534236469), 0); // reversed would overflow
    }

    #[test]
    fn test_overflow_negative() {
        assert_eq!(reverse(-2147483648), 0); // i32::MIN reversed overflows
    }

    #[test]
    fn test_max_no_overflow() {
        assert_eq!(reverse(2147447412), 2147447412);
    }

    #[test]
    fn test_ten() {
        assert_eq!(reverse(10), 1);
    }

    #[test]
    fn test_negative_trailing_zero() {
        assert_eq!(reverse(-120), -21);
    }
}
