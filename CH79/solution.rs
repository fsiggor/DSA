pub fn climb_stairs(n: i32) -> i32 {
    // Implement your solution here
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one_step() {
        assert_eq!(climb_stairs(1), 1);
    }

    #[test]
    fn test_two_steps() {
        assert_eq!(climb_stairs(2), 2);
    }

    #[test]
    fn test_three_steps() {
        assert_eq!(climb_stairs(3), 3);
    }

    #[test]
    fn test_four_steps() {
        assert_eq!(climb_stairs(4), 5);
    }

    #[test]
    fn test_five_steps() {
        assert_eq!(climb_stairs(5), 8);
    }

    #[test]
    fn test_ten_steps() {
        assert_eq!(climb_stairs(10), 89);
    }

    #[test]
    fn test_large() {
        assert_eq!(climb_stairs(45), 1836311903);
    }
}
