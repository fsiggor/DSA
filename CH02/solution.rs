pub fn count_and_say(n: i32) -> String {
    // Implement your solution here
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_n_1() {
        assert_eq!(count_and_say(1), "1");
    }

    #[test]
    fn test_n_2() {
        assert_eq!(count_and_say(2), "11");
    }

    #[test]
    fn test_n_3() {
        assert_eq!(count_and_say(3), "21");
    }

    #[test]
    fn test_n_4() {
        assert_eq!(count_and_say(4), "1211");
    }

    #[test]
    fn test_n_5() {
        assert_eq!(count_and_say(5), "111221");
    }

    #[test]
    fn test_n_6() {
        assert_eq!(count_and_say(6), "312211");
    }

    #[test]
    fn test_n_7() {
        assert_eq!(count_and_say(7), "13112221");
    }

    #[test]
    fn test_n_8() {
        assert_eq!(count_and_say(8), "1113213211");
    }
}
