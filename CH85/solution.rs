pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    // Implement your solution here
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sorted_groups(mut groups: Vec<Vec<String>>) -> Vec<Vec<String>> {
        for group in groups.iter_mut() {
            group.sort();
        }
        groups.sort_by(|a, b| {
            a.len().cmp(&b.len()).then_with(|| a.cmp(b))
        });
        groups
    }

    #[test]
    fn test_example_1() {
        let input: Vec<String> = vec!["eat","tea","tan","ate","nat","bat"]
            .into_iter().map(String::from).collect();
        let result = sorted_groups(group_anagrams(input));
        let expected = sorted_groups(vec![
            vec!["bat".to_string()],
            vec!["nat".to_string(), "tan".to_string()],
            vec!["ate".to_string(), "eat".to_string(), "tea".to_string()],
        ]);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_example_2() {
        let input = vec!["".to_string()];
        let result = group_anagrams(input);
        assert_eq!(result, vec![vec!["".to_string()]]);
    }

    #[test]
    fn test_example_3() {
        let input = vec!["a".to_string()];
        let result = group_anagrams(input);
        assert_eq!(result, vec![vec!["a".to_string()]]);
    }

    #[test]
    fn test_no_anagrams() {
        let input: Vec<String> = vec!["abc","def","ghi"]
            .into_iter().map(String::from).collect();
        let result = group_anagrams(input);
        assert_eq!(result.len(), 3);
        for group in &result {
            assert_eq!(group.len(), 1);
        }
    }

    #[test]
    fn test_all_anagrams() {
        let input: Vec<String> = vec!["abc","bca","cab"]
            .into_iter().map(String::from).collect();
        let result = group_anagrams(input);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].len(), 3);
    }

    #[test]
    fn test_single_char_anagrams() {
        let input: Vec<String> = vec!["a","a","a"]
            .into_iter().map(String::from).collect();
        let result = group_anagrams(input);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].len(), 3);
    }
}
