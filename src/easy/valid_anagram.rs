pub struct Solution {}
impl Solution {
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
    fn test_is_anagram() {
        assert_eq!(
            Solution::is_anagram("anagram".to_string(), "nagaram".to_string()),
            true
        );
        assert_eq!(
            Solution::is_anagram("rat".to_string(), "car".to_string()),
            false
        );
    }
}
