use std::collections::HashSet;

pub struct Solution {}
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        if s.len() < 2 {
            return s.len() as i32;
        }

        let mut set: HashSet<char> = HashSet::new();
        let chars: Vec<char> = s.chars().collect();

        let mut j = 0;
        set.insert(chars[j]);
        let mut max = 1;

        for i in 0..chars.len() {
            while j < chars.len() - 1 && !set.contains(&chars[j + 1]) {
                j += 1;
                set.insert(chars[j]);

                if j as i32 - i as i32 + 1 > max {
                    max = j as i32 - i as i32 + 1;
                }
            }

            if j == chars.len() - 1 {
                break;
            }

            set.remove(&chars[i]);
        }

        max
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(0, Solution::length_of_longest_substring("".to_string()));
    }

    #[test]
    fn test_2() {
        assert_eq!(1, Solution::length_of_longest_substring("a".to_string()));
    }

    #[test]
    fn test_3() {
        assert_eq!(
            3,
            Solution::length_of_longest_substring("abcabcbb".to_string())
        );
    }

    #[test]
    fn test_4() {
        assert_eq!(1, Solution::length_of_longest_substring("bbbb".to_string()));
    }

    #[test]
    fn test_5() {
        assert_eq!(
            3,
            Solution::length_of_longest_substring("pwwkew".to_string())
        );
    }
}
