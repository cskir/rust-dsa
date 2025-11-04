pub struct Solution {}
impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut groups: Vec<String> = Vec::new();
        let mut result: Vec<Vec<String>> = Vec::new();

        for s in strs {
            let mut found = false;
            for (i, key) in groups.iter().enumerate() {
                if Solution::is_anagram(s.clone(), key.clone()) {
                    result[i].push(s.clone());
                    found = true;
                    break;
                }
            }

            if !found {
                groups.push(s.clone());
                result.push(vec![s.clone()]);
            }
        }

        result
    }

    pub fn group_anagrams2(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut map: std::collections::HashMap<String, Vec<String>> =
            std::collections::HashMap::new();

        for s in strs {
            let mut chars: Vec<char> = s.chars().collect();
            chars.sort_unstable();
            let key: String = chars.into_iter().collect();

            map.entry(key).or_insert(Vec::new()).push(s);
        }

        map.into_values().collect()
    }

    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }

        let mut s_ch_counts = std::collections::HashMap::new();

        for ch in s.chars() {
            *s_ch_counts.entry(ch).or_insert(0) += 1;
        }

        for ch in t.chars() {
            match s_ch_counts.get_mut(&ch) {
                Some(count) => {
                    *count -= 1;
                    if *count < 0 {
                        return false;
                    }
                }
                None => return false,
            }
        }

        true
    }
}

#[cfg(test)]
pub mod tests {
    use super::Solution;

    #[test]
    fn test_group_anagrams() {
        let input = vec![
            "eat".to_string(),
            "tea".to_string(),
            "tan".to_string(),
            "ate".to_string(),
            "nat".to_string(),
            "bat".to_string(),
        ];
        let mut result = Solution::group_anagrams(input);
        for group in &mut result {
            group.sort();
        }
        result.sort_by(|a, b| a[0].cmp(&b[0]));

        let expected = vec![
            vec!["ate".to_string(), "eat".to_string(), "tea".to_string()],
            vec!["bat".to_string()],
            vec!["nat".to_string(), "tan".to_string()],
        ];

        assert_eq!(result, expected);
    }

    #[test]
    fn test_group_anagrams_empty() {
        let input = vec!["".to_string()];
        let result = Solution::group_anagrams(input);
        let expected = vec![vec!["".to_string()]];

        assert_eq!(result, expected);
    }

    #[test]
    fn test_group_anagrams_char() {
        let input = vec!["a".to_string()];
        let result = Solution::group_anagrams(input);
        let expected = vec![vec!["a".to_string()]];

        assert_eq!(result, expected);
    }
}
