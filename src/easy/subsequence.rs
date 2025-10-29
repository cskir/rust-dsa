pub struct Solution {}

impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        if s.len() == 0 {
            return true;
        }

        let char_s: Vec<char> = s.chars().collect();
        let char_t: Vec<char> = t.chars().collect();
        let mut idx_s = 0;

        for idx_t in 0..char_t.len() {
            if char_s[idx_s] == char_t[idx_t] {
                if idx_s == char_s.len() - 1 {
                    return true;
                }
                idx_s += 1;
            }
        }
        false
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1() {
        assert!(Solution::is_subsequence(
            "abc".to_string(),
            "ahbgdc".to_string()
        ));
    }

    #[test]
    fn test_2() {
        assert!(!Solution::is_subsequence(
            "axc".to_string(),
            "ahbgdc".to_string()
        ));
    }

    #[test]
    fn test_3() {
        assert!(Solution::is_subsequence(
            "".to_string(),
            "ahbgdc".to_string()
        ));
    }

    #[test]
    fn test_4() {
        assert!(Solution::is_subsequence("".to_string(), "".to_string()));
    }

    #[test]
    fn test_5() {
        assert!(!Solution::is_subsequence("a".to_string(), "".to_string()));
    }
}
