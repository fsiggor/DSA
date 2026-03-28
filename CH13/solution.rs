pub fn num_decodings(s: String) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_12() {
        assert_eq!(num_decodings("12".to_string()), 2);
    }

    #[test]
    fn test_226() {
        assert_eq!(num_decodings("226".to_string()), 3);
    }

    #[test]
    fn test_leading_zero() {
        assert_eq!(num_decodings("06".to_string()), 0);
    }

    #[test]
    fn test_single_digit() {
        assert_eq!(num_decodings("1".to_string()), 1);
    }

    #[test]
    fn test_zero() {
        assert_eq!(num_decodings("0".to_string()), 0);
    }

    #[test]
    fn test_10() {
        assert_eq!(num_decodings("10".to_string()), 1);
    }

    #[test]
    fn test_27() {
        assert_eq!(num_decodings("27".to_string()), 1);
    }

    #[test]
    fn test_1111() {
        assert_eq!(num_decodings("1111".to_string()), 5);
    }

    #[test]
    fn test_100() {
        assert_eq!(num_decodings("100".to_string()), 0);
    }

    #[test]
    fn test_2101() {
        assert_eq!(num_decodings("2101".to_string()), 1);
    }
}

fn main() {}
