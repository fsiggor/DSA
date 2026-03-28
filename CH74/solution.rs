pub fn unique_paths(m: i32, n: i32) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3x7() {
        assert_eq!(unique_paths(3, 7), 28);
    }

    #[test]
    fn test_3x2() {
        assert_eq!(unique_paths(3, 2), 3);
    }

    #[test]
    fn test_1x1() {
        assert_eq!(unique_paths(1, 1), 1);
    }

    #[test]
    fn test_1xn() {
        assert_eq!(unique_paths(1, 5), 1);
    }

    #[test]
    fn test_mx1() {
        assert_eq!(unique_paths(5, 1), 1);
    }

    #[test]
    fn test_2x2() {
        assert_eq!(unique_paths(2, 2), 2);
    }

    #[test]
    fn test_7x3() {
        assert_eq!(unique_paths(7, 3), 28);
    }

    #[test]
    fn test_large() {
        assert_eq!(unique_paths(10, 10), 48620);
    }
}

fn main() {}
