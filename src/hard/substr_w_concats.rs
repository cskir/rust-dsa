use std::collections::HashMap;

pub struct Solution {}
impl Solution {
    pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
        // words.len > 0
        let step = words[0].len();
        let mut map_ref: HashMap<String, i32> = HashMap::new();

        for i in 0..words.len() {
            let value = map_ref.get_mut(&words[i]);

            if value.is_some() {
                *value.unwrap() += 1;
            } else {
                map_ref.insert(words[i].clone(), 1);
            }
        }
        let mut map = map_ref.clone();

        let mut res: Vec<i32> = Vec::new();

        println!("{:?}", s);
        println!("{:?}", map);
        for offset in 0..step {
            println!("offset {} ", offset);
            let mut i = offset;
            map = map_ref.clone();
            let mut session_start: Option<usize> = None;
            while i + step <= s.len() {
                if session_start.is_none() {
                    session_start = Some(i);
                }

                println!("while {} {:?} {}", i, session_start, &s[i..(i + step)]);
                match map.get_mut(&s[i..(i + step)]) {
                    Some(value) => {
                        *value -= 1;

                        match Solution::check(&map) {
                            -1 => {
                                i = session_start.unwrap();
                                println!(" -1 {} {:?}", i, map);
                                map = map_ref.clone();
                                session_start = None;
                            }
                            1 => {
                                //i -= (map.len() - 1) * step;
                                i = session_start.unwrap();
                                res.push(i as i32);
                                map = map_ref.clone();
                                session_start = None;
                                println!(" 1  {:?}", i);
                            }
                            _ => {
                                println!(" 0 {:?}", map);
                            }
                        }
                    }
                    None => {
                        map = map_ref.clone();
                        session_start = None;
                        println!(" None {:?} {:?}", session_start, map);
                    }
                }

                i += step;
                println!(" Next i {}", i);
            }
        }

        res
    }

    fn check(map: &HashMap<String, i32>) -> i8 {
        let mut res = 0;
        for value in map.values() {
            if *value < 0 {
                return -1;
            }

            res += *value;
        }
        if res == 0 { 1 } else { 0 }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn convert_from_str(strs: Vec<&'static str>) -> Vec<String> {
        strs.iter().map(|i| i.to_string()).collect()
    }

    #[test]
    fn test_1() {
        let words = vec!["foo", "bar"];

        assert_eq!(
            vec![0, 9],
            Solution::find_substring("barfoothefoobarman".to_string(), convert_from_str(words))
        );
    }

    #[test]
    fn test_2() {
        let words = vec!["word", "good", "best", "word"];

        let expceted: Vec<i32> = vec![];
        assert_eq!(
            expceted,
            Solution::find_substring(
                "wordgoodgoodgoodbestword".to_string(),
                convert_from_str(words)
            )
        );
    }

    #[test]
    fn test_3() {
        let words = vec!["bar", "foo", "the"];

        assert_eq!(
            vec![6, 9, 12],
            Solution::find_substring(
                "barfoofoobarthefoobarman".to_string(),
                convert_from_str(words)
            )
        );
    }

    #[test]
    fn test_4() {
        let words = vec!["word", "good", "best", "good"];

        assert_eq!(
            vec![8],
            Solution::find_substring(
                "wordgoodgoodgoodbestword".to_string(),
                convert_from_str(words)
            )
        );
    }

    #[test]
    fn test_5() {
        let words = vec!["ababa", "babab"];

        assert_eq!(
            vec![0],
            Solution::find_substring("ababababab".to_string(), convert_from_str(words))
        );
    }

    #[test]
    fn test_6() {
        let words = vec!["a", "a", "a", "a", "a", "a", "a", "a", "a", "a"];

        assert_eq!(
            vec![0],
            Solution::find_substring("aaaaaaaaaa".to_string(), convert_from_str(words))
        );
    }
}
