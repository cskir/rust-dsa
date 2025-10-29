use regex::Regex;
pub struct Solution {}

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let mut j = 0;

        let mut chars: Vec<char> = s.chars().collect();
        for i in 0..s.len() {
            if chars[i].is_alphanumeric() {
                chars[j] = chars[i].to_ascii_lowercase();
                j += 1;
            }
        }

        for i in 0..j / 2 {
            if chars[i] != chars[j - 1 - i] {
                return false;
            }
        }
        true
    }

    pub fn is_palindrome2(s: String) -> bool {
        let rgx = Regex::new("[^a-zA-Z0-9]").unwrap();
        let s = rgx.replace_all(&s, "");

        let mut i_fw = s.chars();
        let mut i_bw = s.chars().rev();

        for _ in 0..s.len() / 2 {
            if i_fw.next().unwrap().to_ascii_lowercase()
                != i_bw.next().unwrap().to_ascii_lowercase()
            {
                return false;
            }
        }
        true
    }
}
