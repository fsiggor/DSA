pub fn generate_parenthesis(n: i32) -> Vec<String> {
    // Implement your solution here
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_n_1() {
        assert_eq!(generate_parenthesis(1), vec!["()"]);
    }

    #[test]
    fn test_n_2() {
        let mut result = generate_parenthesis(2);
        result.sort();
        assert_eq!(result, vec!["(())", "()()"]);
    }

    #[test]
    fn test_n_3() {
        let mut result = generate_parenthesis(3);
        result.sort();
        assert_eq!(result, vec!["((()))", "(()())", "(())()", "()(())", "()()()"]);
    }

    #[test]
    fn test_n_3_count() {
        assert_eq!(generate_parenthesis(3).len(), 5);
    }

    #[test]
    fn test_n_4_count() {
        assert_eq!(generate_parenthesis(4).len(), 14);
    }

    #[test]
    fn test_all_valid() {
        let results = generate_parenthesis(3);
        for s in &results {
            let mut count = 0i32;
            for c in s.chars() {
                if c == '(' { count += 1; } else { count -= 1; }
                assert!(count >= 0, "Invalid parentheses: {}", s);
            }
            assert_eq!(count, 0, "Unbalanced parentheses: {}", s);
        }
    }
}
