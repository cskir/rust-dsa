pub struct Solution {}

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut res = String::new();
        let mut iters = Vec::new();

        for i in 0..strs.len() {
            iters.push(strs[i].chars());
        }

        let mut tmp: Option<char>;
        let mut exit: bool;
        loop {
            tmp = None;
            exit = false;
            for i in 0..strs.len() {
                match iters[i].next() {
                    None => {
                        exit = true;
                        break;
                    }
                    Some(c) => match tmp {
                        None => tmp = Some(c),
                        Some(s) => {
                            if s != c {
                                exit = true;
                                break;
                            }
                        }
                    },
                }
            }

            if exit {
                break;
            } else {
                res.push(tmp.unwrap());
            }
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let words = vec!["".to_string(), "".to_string(), "".to_string()];
        let result = Solution::longest_common_prefix(words);
        assert_eq!("", result);
    }

    #[test]
    fn test_2() {
        let words = vec!["a".to_string(), "ab".to_string(), "abc".to_string()];
        let result = Solution::longest_common_prefix(words);
        assert_eq!("a", result);
    }

    #[test]
    fn test_3() {
        let words = vec![
            "flower".to_string(),
            "flow".to_string(),
            "flight".to_string(),
        ];
        let result = Solution::longest_common_prefix(words);
        assert_eq!("fl", result);
    }

    #[test]
    fn test_4() {
        let words = vec!["dog".to_string(), "racecar".to_string(), "car".to_string()];
        let result = Solution::longest_common_prefix(words);
        assert_eq!("", result);
    }

    #[test]
    fn test_5() {
        let words = vec!["race".to_string(), "racecar".to_string(), "car".to_string()];
        let result = Solution::longest_common_prefix(words);
        assert_eq!("", result);
    }
}
