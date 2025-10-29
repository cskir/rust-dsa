use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        println!("s: {}, t: {}", s, t);

        let (rule_map, mut map) = Self::create_maps(&t);

        println!("rule_map: {:?}", rule_map);
        let chars: Vec<char> = s.chars().collect();
        let mut left = 0;
        let mut right;
        let mut found = false;
        let mut pair: (usize, usize) = (0, chars.len() + 1);

        for i in 0..chars.len() {
            if rule_map.contains_key(&chars[i]) {
                right = i;
                let act_value = map.get_mut(&chars[right]);

                if act_value.is_some() {
                    *act_value.unwrap() += 1;
                } else {
                    map.insert(chars[right], 1);
                }

                println!("right: {} map: {:?}", right, map);
                if Solution::fill_map(&map, &rule_map) {
                    found = true;
                    println!(
                        "right filled : {:?}, {} - {}, {:?} ",
                        map, left, right, pair
                    );
                    if right - left < pair.1 - pair.0 {
                        pair = (left, right);
                    }

                    while left <= right {
                        if rule_map.contains_key(&chars[left]) {
                            println!("left: {} map: {:?}", left, map);
                            if Solution::fill_map(&map, &rule_map) {
                                println!(
                                    "left filled : {:?}, {} - {}, {:?} ",
                                    map, left, right, pair
                                );
                                if right - left < pair.1 - pair.0 {
                                    pair = (left, right);
                                }

                                if let Some(value) = map.get_mut(&chars[left]) {
                                    *value -= 1;
                                }
                            } else {
                                break;
                            }
                        }

                        left += 1;
                    }
                }
            }
        }
        if found {
            s[pair.0..=pair.1].to_string()
        } else {
            String::new()
        }
    }

    fn create_maps(t: &String) -> (HashMap<char, i32>, HashMap<char, i32>) {
        let mut rule_map: HashMap<char, i32> = HashMap::new();
        let mut map: HashMap<char, i32> = HashMap::new();
        let chars: Vec<char> = t.chars().collect();

        for i in 0..chars.len() {
            let value = rule_map.get_mut(&chars[i]);

            if value.is_some() {
                *value.unwrap() += 1;
            } else {
                rule_map.insert(chars[i], 1);
                map.insert(chars[i], 0);
            }
        }

        (rule_map, map)
    }

    fn fill_map(map: &HashMap<char, i32>, rule_map: &HashMap<char, i32>) -> bool {
        return !Solution::not_fill_map(map, rule_map);
    }

    fn not_fill_map(map: &HashMap<char, i32>, rule_map: &HashMap<char, i32>) -> bool {
        for (key, limit_value) in rule_map.iter() {
            if *map.get(key).unwrap() < *limit_value {
                return true;
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            "a".to_string(),
            Solution::min_window("a".to_string(), "a".to_string())
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            "".to_string(),
            Solution::min_window("a".to_string(), "b".to_string())
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            "".to_string(),
            Solution::min_window("y".to_string(), "yy".to_string())
        );
    }

    #[test]
    fn test_4() {
        assert_eq!(
            "BANC".to_string(),
            Solution::min_window("ADOBECODEBANC".to_string(), "ABC".to_string())
        );
    }

    #[test]
    fn test_5() {
        assert_eq!(
            "baa".to_string(),
            Solution::min_window("bbaac".to_string(), "aba".to_string())
        );
    }

    #[test]
    fn test_6() {
        assert_eq!(
            "abbbbbcdd".to_string(),
            Solution::min_window("aaaaaaaaaaaabbbbbcdd".to_string(), "abcdd".to_string())
        );
    }

    #[test]
    fn test_7() {
        assert_eq!(
            "tcnvhxqgndyozpcigzykbiaucyvwrjvknifufxducbkbsmlanl".to_string(),
            Solution::min_window("wegdtzwabazduwwdysdetrrctotpcepalxdewzezbfewbabbseinxbqqplitpxtcwwhuyntbtzxwzyaufihclztckdwccpeyonumbpnuonsnnsjscrvpsqsftohvfnvtbphcgxyumqjzltspmphefzjypsvugqqjhzlnylhkdqmolggxvneaopadivzqnpzurmhpxqcaiqruwztroxtcnvhxqgndyozpcigzykbiaucyvwrjvknifufxducbkbsmlanllpunlyohwfsssiazeixhebipfcdqdrcqiwftutcrbxjthlulvttcvdtaiwqlnsdvqkrngvghupcbcwnaqiclnvnvtfihylcqwvderjllannflchdklqxidvbjdijrnbpkftbqgpttcagghkqucpcgmfrqqajdbynitrbzgwukyaqhmibpzfxmkoeaqnftnvegohfudbgbbyiqglhhqevcszdkokdbhjjvqqrvrxyvvgldtuljygmsircydhalrlgjeyfvxdstmfyhzjrxsfpcytabdcmwqvhuvmpssingpmnpvgmpletjzunewbamwiirwymqizwxlmojsbaehupiocnmenbcxjwujimthjtvvhenkettylcoppdveeycpuybekulvpgqzmgjrbdrmficwlxarxegrejvrejmvrfuenexojqdqyfmjeoacvjvzsrqycfuvmozzuypfpsvnzjxeazgvibubunzyuvugmvhguyojrlysvxwxxesfioiebidxdzfpumyon".to_string(), "ozgzyywxvtublcl".to_string())
        );
    }

    #[test]
    fn test_8() {
        assert_eq!(
            "cabd".to_string(),
            Solution::min_window("abcabdebac".to_string(), "cda".to_string())
        );
    }

    #[test]
    fn test_9() {
        assert_eq!(
            "b".to_string(),
            Solution::min_window("ab".to_string(), "b".to_string())
        );
    }
}
