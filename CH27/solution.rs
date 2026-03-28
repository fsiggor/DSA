pub fn daily_temperatures(temps: Vec<i32>) -> Vec<i32> {
    // Implement your solution here
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(
            daily_temperatures(vec![73, 74, 75, 71, 69, 72, 76, 73]),
            vec![1, 1, 4, 2, 1, 1, 0, 0]
        );
    }

    #[test]
    fn test_example_2() {
        assert_eq!(
            daily_temperatures(vec![30, 40, 50, 60]),
            vec![1, 1, 1, 0]
        );
    }

    #[test]
    fn test_example_3() {
        assert_eq!(daily_temperatures(vec![30, 60, 90]), vec![1, 1, 0]);
    }

    #[test]
    fn test_single_element() {
        assert_eq!(daily_temperatures(vec![50]), vec![0]);
    }

    #[test]
    fn test_decreasing_temps() {
        assert_eq!(daily_temperatures(vec![90, 80, 70, 60]), vec![0, 0, 0, 0]);
    }

    #[test]
    fn test_all_same() {
        assert_eq!(daily_temperatures(vec![50, 50, 50, 50]), vec![0, 0, 0, 0]);
    }

    #[test]
    fn test_increasing_then_decreasing() {
        assert_eq!(
            daily_temperatures(vec![30, 50, 70, 60, 40]),
            vec![1, 1, 0, 0, 0]
        );
    }

    #[test]
    fn test_two_elements_warmer() {
        assert_eq!(daily_temperatures(vec![30, 40]), vec![1, 0]);
    }

    #[test]
    fn test_two_elements_cooler() {
        assert_eq!(daily_temperatures(vec![40, 30]), vec![0, 0]);
    }
}
