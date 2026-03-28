pub fn roman_to_int(s: String) -> i32 {
    // Implement your solution here
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iii() {
        assert_eq!(roman_to_int("III".to_string()), 3);
    }

    #[test]
    fn test_iv() {
        assert_eq!(roman_to_int("IV".to_string()), 4);
    }

    #[test]
    fn test_ix() {
        assert_eq!(roman_to_int("IX".to_string()), 9);
    }

    #[test]
    fn test_lviii() {
        assert_eq!(roman_to_int("LVIII".to_string()), 58);
    }

    #[test]
    fn test_mcmxciv() {
        assert_eq!(roman_to_int("MCMXCIV".to_string()), 1994);
    }

    #[test]
    fn test_mmmcmxcix() {
        assert_eq!(roman_to_int("MMMCMXCIX".to_string()), 3999);
    }

    #[test]
    fn test_single() {
        assert_eq!(roman_to_int("D".to_string()), 500);
    }
}
