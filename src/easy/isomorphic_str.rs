use std::collections::HashMap;

pub struct Solution {}
impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }

        let mut map_s: HashMap<char, char> = HashMap::new();
        let mut map_t: HashMap<char, char> = HashMap::new();
        let s_chars: Vec<char> = s.chars().collect();
        let t_chars: Vec<char> = t.chars().collect();

        for i in 0..s.len() {
            if map_s.contains_key(&s_chars[i]) {
                if map_s[&s_chars[i]] != t_chars[i] {
                    return false;
                }
            } else {
                map_s.insert(s_chars[i], t_chars[i]);
            }

            if map_t.contains_key(&t_chars[i]) {
                if map_t[&t_chars[i]] != s_chars[i] {
                    return false;
                }
            } else {
                map_t.insert(t_chars[i], s_chars[i]);
            }
        }

        true
    }
}

#[cfg(test)]
pub mod tests {
    use super::Solution;

    #[test]
    fn test_is_isomorphic() {
        assert_eq!(
            Solution::is_isomorphic("egg".to_string(), "add".to_string()),
            true
        );
        assert_eq!(
            Solution::is_isomorphic("foo".to_string(), "bar".to_string()),
            false
        );
        assert_eq!(
            Solution::is_isomorphic("paper".to_string(), "title".to_string()),
            true
        );
    }
}
