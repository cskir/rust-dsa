pub struct Solution {}
impl Solution {
    pub fn word_pattern(pattern: String, s: String) -> bool {
        let words: Vec<&str> = s.split(' ').collect();
        if pattern.len() != words.len() {
            return false;
        }

        let mut char_to_word = std::collections::HashMap::new();
        let mut word_to_char = std::collections::HashMap::new();

        for (index, ch) in pattern.chars().enumerate() {
            let word = words[index];
            if word_to_char.contains_key(word) {
                if word_to_char[word] != ch {
                    return false;
                }
            } else {
                word_to_char.insert(word, ch);
            }

            if char_to_word.contains_key(&ch) {
                if char_to_word[&ch] != word {
                    return false;
                }
            } else {
                char_to_word.insert(ch, word);
            }
        }

        true
    }
}

#[cfg(test)]
pub mod tests {
    use super::Solution;

    #[test]
    fn test_word_pattern() {
        assert_eq!(
            Solution::word_pattern("abba".to_string(), "dog cat cat dog".to_string()),
            true
        );
        assert_eq!(
            Solution::word_pattern("abba".to_string(), "dog cat cat fish".to_string()),
            false
        );
        assert_eq!(
            Solution::word_pattern("aaaa".to_string(), "dog cat cat dog".to_string()),
            false
        );
        assert_eq!(
            Solution::word_pattern("abba".to_string(), "dog dog dog dog".to_string()),
            false
        );
    }
}
