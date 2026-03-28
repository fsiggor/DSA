pub fn title_to_number(column_title: String) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a() {
        assert_eq!(title_to_number("A".to_string()), 1);
    }

    #[test]
    fn test_b() {
        assert_eq!(title_to_number("B".to_string()), 2);
    }

    #[test]
    fn test_z() {
        assert_eq!(title_to_number("Z".to_string()), 26);
    }

    #[test]
    fn test_ab() {
        assert_eq!(title_to_number("AB".to_string()), 28);
    }

    #[test]
    fn test_zy() {
        assert_eq!(title_to_number("ZY".to_string()), 701);
    }

    #[test]
    fn test_aa() {
        assert_eq!(title_to_number("AA".to_string()), 27);
    }

    #[test]
    fn test_az() {
        assert_eq!(title_to_number("AZ".to_string()), 52);
    }

    #[test]
    fn test_ba() {
        assert_eq!(title_to_number("BA".to_string()), 53);
    }

    #[test]
    fn test_aaa() {
        assert_eq!(title_to_number("AAA".to_string()), 703);
    }

    #[test]
    fn test_single_letters() {
        assert_eq!(title_to_number("C".to_string()), 3);
        assert_eq!(title_to_number("M".to_string()), 13);
    }
}
