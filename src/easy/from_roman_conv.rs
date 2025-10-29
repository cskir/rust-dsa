use std::collections::{HashMap, HashSet};

pub struct Solution {}

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let char_value = HashMap::from([
            ('I', 1),
            ('V', 5),
            ('X', 10),
            ('L', 50),
            ('C', 100),
            ('D', 500),
            ('M', 1000),
        ]);

        let mut res = 0;
        let chars: Vec<char> = s.chars().collect();

        for i in 0..chars.len() {
            if i < chars.len() - 1 && char_value[&chars[i]] < char_value[&chars[i + 1]] {
                res -= char_value[&chars[i]];
            } else {
                res += char_value[&chars[i]];
            }
        }
        res
    }

    pub fn roman_to_int2(s: String) -> i32 {
        let char_value = HashMap::from([
            ('I', 1),
            ('V', 5),
            ('X', 10),
            ('L', 50),
            ('C', 100),
            ('D', 500),
            ('M', 1000),
        ]);
        let char_dup = HashMap::from([
            ('I', HashSet::from(['V', 'X'])),
            ('X', HashSet::from(['L', 'C'])),
            ('C', HashSet::from(['D', 'M'])),
        ]);

        let mut res = 0;
        let chars: Vec<char> = s.chars().collect();

        for i in 0..chars.len() {
            let mul = match char_dup.get(&chars[i]) {
                None => 1,
                Some(set) => {
                    if i < chars.len() - 1 && set.contains(&chars[i + 1]) {
                        -1
                    } else {
                        1
                    }
                }
            };
            res += mul * char_value[&chars[i]];
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(1, Solution::roman_to_int("I".to_string()));
        assert_eq!(2, Solution::roman_to_int("II".to_string()));
        assert_eq!(4, Solution::roman_to_int("IV".to_string()));
        assert_eq!(5, Solution::roman_to_int("V".to_string()));
        assert_eq!(6, Solution::roman_to_int("VI".to_string()));
        assert_eq!(7, Solution::roman_to_int("VII".to_string()));

        assert_eq!(2830, Solution::roman_to_int("MMDCCCXXX".to_string()));
        assert_eq!(682, Solution::roman_to_int("DCLXXXII".to_string()));
        assert_eq!(2779, Solution::roman_to_int("MMDCCLXXIX".to_string()));
        assert_eq!(528, Solution::roman_to_int("DXXVIII".to_string()));

        assert_eq!(1662, Solution::roman_to_int("MDCLXII".to_string()));
    }
}
