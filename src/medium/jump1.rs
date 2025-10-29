pub struct Solution {}

impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        if nums.len() == 0 {
            return false;
        }
        if nums.len() == 1 {
            return true;
        }

        let mut targets = vec![nums.len() - 1];

        for i in (0..(nums.len() - 1)).rev() {
            let mut is_target = false;
            for j in (0..targets.len()).rev() {
                if i + nums[i] as usize >= targets[j] {
                    is_target = true;
                    break;
                }
            }
            if is_target {
                targets.push(i);
            }
        }

        match targets.pop() {
            None => false,
            Some(index) => index == 0,
        }
    }

    pub fn can_jump2(nums: Vec<i32>) -> bool {
        if nums.len() == 1 && nums[0] == 0 {
            return true;
        }
        let mut store: Vec<Option<bool>> = vec![None; nums.len()];
        Self::can_jump_partial(&nums[..], &mut store, 0)
    }

    pub fn all_false(store: &mut Vec<Option<bool>>, start: usize, end: usize) -> bool {
        let mut all_false = true;
        for j in start..end {
            all_false &= match store[j] {
                None => false,
                Some(value) => !value,
            };
        }
        return !all_false;
    }

    pub fn can_jump_partial(nums: &[i32], store: &mut Vec<Option<bool>>, offset: usize) -> bool {
        Self::log(offset, format!("{:?}", nums));
        for i in 0..nums.len() {
            if nums[i] == 0 && i < nums.len() - 1 {
                return false;
            }

            // if Self::all_false(store, i, store.len() - 1) {
            //     return false;
            // }

            for j in (1..=(nums[i] as usize)).rev() {
                // végére ugrunk? -> nem
                // összes közbülső false -> nem
                let index = offset + i + j;
                Self::log(offset, format!("i{} j{} index: {}", i, j, index));
                if index >= store.len() - 1 {
                    return true;
                }

                Self::log(offset, format!(" value: {:?}", store[index]));
                let reach = match store[index] {
                    Some(reach) => reach,
                    None => {
                        println!("call: {} offset", index);
                        let reach = Self::can_jump_partial(&nums[(i + j)..], store, index);
                        store[index] = Some(reach);
                        Self::log(offset, format!("{:?}", store));
                        reach
                    }
                };
                if reach {
                    return true;
                }
            }
        }
        false
    }

    fn log(offset: usize, msg: String) {
        println!("{:?}{}", vec![""; offset], msg);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let nums = vec![7, 1, 5, 3, 6, 4];
        let result = Solution::can_jump(nums);
        assert_eq!(true, result);
    }

    #[test]
    fn test_2() {
        let nums = vec![2, 3, 1, 1, 4];
        let result = Solution::can_jump(nums);
        assert_eq!(true, result);
    }

    #[test]
    fn test_3() {
        let nums = vec![3, 2, 1, 0, 4];
        let result = Solution::can_jump(nums);
        assert_eq!(false, result);
    }

    #[test]
    fn test_4() {
        let nums = vec![7, 1, 5, 3, 6, 0];
        let result = Solution::can_jump(nums);
        assert_eq!(true, result);
    }

    #[test]
    fn test_5() {
        let nums = vec![0, 1, 5, 3, 6, 0];
        let result = Solution::can_jump(nums);
        assert_eq!(false, result);
    }

    #[test]
    fn test_6() {
        let nums = vec![0];
        let result = Solution::can_jump(nums);
        assert_eq!(true, result);
    }

    #[test]
    fn test_7() {
        let nums = vec![1, 0];
        let result = Solution::can_jump(nums);
        assert_eq!(true, result);
    }

    #[test]
    fn test_8() {
        let nums = vec![2, 0];
        let result = Solution::can_jump(nums);
        assert_eq!(true, result);
    }

    #[test]
    fn test_9() {
        let nums = vec![2, 0, 2, 0, 2, 0, 0];
        let result = Solution::can_jump(nums);
        assert_eq!(true, result);
    }

    #[test]
    fn test_10() {
        let nums = vec![2, 0, 2, 0, 1, 0, 0];
        let result = Solution::can_jump(nums);
        assert_eq!(false, result);
    }

    #[test]
    fn test_11() {
        let nums = vec![10, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0, 0];
        let result = Solution::can_jump(nums);
        assert_eq!(false, result);
    }
}
