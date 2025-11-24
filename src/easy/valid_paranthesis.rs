use std::collections::HashMap;

pub struct Solution {}
impl Solution {
    pub fn is_valid(s: String) -> bool {
        let map: HashMap<char, char> = HashMap::from([(')', '('), ('}', '{'), (']', '[')]);
        let chars: Vec<char> = s.chars().collect();
        let mut tmp: Vec<char> = Vec::new();
        for i in 0..chars.len() {
            if map.contains_key(&chars[i]) {
                if tmp.len() == 0 || tmp.pop().unwrap() != map[&chars[i]] {
                    return false;
                }
            } else {
                tmp.push(chars[i]);
            }
        }

        return tmp.len() == 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let input = String::from("()");
        let result = Solution::is_valid(input);
        assert_eq!(result, true);
    }

    #[test]
    fn test_2() {
        let input = String::from("()[]{}");
        let result = Solution::is_valid(input);
        assert_eq!(result, true);
    }

    #[test]
    fn test_3() {
        let input = String::from("(]");
        let result = Solution::is_valid(input);
        assert_eq!(result, false);
    }

    #[test]
    fn test_4() {
        let input = String::from("([)]");
        let result = Solution::is_valid(input);
        assert_eq!(result, false);
    }

    #[test]
    fn test_5() {
        let input = String::from("{[]}");
        let result = Solution::is_valid(input);
        assert_eq!(result, true);
    }

    #[test]
    fn test_6() {
        let input = String::from("[");
        let result = Solution::is_valid(input);
        assert_eq!(result, false);
    }
}
