pub fn count_bits(n: i32) -> Vec<i32> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zero() {
        assert_eq!(count_bits(0), vec![0]);
    }

    #[test]
    fn test_two() {
        assert_eq!(count_bits(2), vec![0, 1, 1]);
    }

    #[test]
    fn test_five() {
        assert_eq!(count_bits(5), vec![0, 1, 1, 2, 1, 2]);
    }

    #[test]
    fn test_one() {
        assert_eq!(count_bits(1), vec![0, 1]);
    }

    #[test]
    fn test_eight() {
        assert_eq!(count_bits(8), vec![0, 1, 1, 2, 1, 2, 2, 3, 1]);
    }

    #[test]
    fn test_length() {
        let result = count_bits(10);
        assert_eq!(result.len(), 11);
    }

    #[test]
    fn test_first_element_always_zero() {
        assert_eq!(count_bits(100)[0], 0);
    }

    #[test]
    fn test_powers_of_two_have_one_bit() {
        let result = count_bits(16);
        assert_eq!(result[1], 1);
        assert_eq!(result[2], 1);
        assert_eq!(result[4], 1);
        assert_eq!(result[8], 1);
        assert_eq!(result[16], 1);
    }

    #[test]
    fn test_fifteen() {
        let result = count_bits(15);
        assert_eq!(result[15], 4); // 1111 in binary
    }
}
