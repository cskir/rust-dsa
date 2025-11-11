use std::collections::HashMap;
pub struct Solution;

impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        let mut map: HashMap<i32, usize> = HashMap::new();

        for (idx, num) in nums.iter().enumerate() {
            match map.get(num) {
                Some(&stored_idx) => {
                    if (idx - stored_idx) as i32 <= k {
                        return true;
                    }
                    *map.get_mut(num).unwrap() = idx;
                }
                _ => {
                    map.insert(*num, idx);
                }
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
        let nums = vec![1, 2, 3, 1];
        let k = 3;
        assert!(Solution::contains_nearby_duplicate(nums, k));
    }
    #[test]
    fn test_2() {
        let nums = vec![1, 0, 1, 1];
        let k = 1;
        assert!(Solution::contains_nearby_duplicate(nums, k));
    }

    #[test]
    fn test_3() {
        let nums = vec![1, 2, 3, 1, 2, 3];
        let k = 2;
        assert!(!Solution::contains_nearby_duplicate(nums, k));
    }
}
