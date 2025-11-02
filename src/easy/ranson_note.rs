use std::collections::HashMap;

pub struct Solution {}
impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        let mut magazine: HashMap<char, i32> =
            magazine.chars().fold(HashMap::new(), |mut acc, c| {
                *acc.entry(c).or_insert(0) += 1;
                acc
            });

        for c in ransom_note.chars() {
            match magazine.get_mut(&c) {
                Some(count) if *count > 0 => *count -= 1,
                _ => return false,
            }
        }

        true
    }
}

#[cfg(test)]
pub mod tests {
    use super::Solution;

    #[test]
    fn test_can_construct() {
        assert_eq!(
            Solution::can_construct("a".to_string(), "b".to_string()),
            false
        );
        assert_eq!(
            Solution::can_construct("aa".to_string(), "ab".to_string()),
            false
        );
        assert_eq!(
            Solution::can_construct("aa".to_string(), "aab".to_string()),
            true
        );
    }
}
