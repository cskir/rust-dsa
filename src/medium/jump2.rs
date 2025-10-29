pub struct Solution {}

impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        //nums.len() > 0
        if nums.len() == 1 {
            return 0;
        }

        let mut targets = vec![nums.len() - 1];
        let mut targets_min_jumps = vec![0];

        for i in (0..(nums.len() - 1)).rev() {
            let mut is_target = false;
            let mut min_jump = 0;
            for j in (0..targets.len()).rev() {
                if i + nums[i] as usize >= targets[j] {
                    is_target = true;
                    if min_jump == 0 || min_jump > targets_min_jumps[j] + 1 {
                        min_jump = targets_min_jumps[j] + 1;
                    }
                }
            }
            if is_target {
                targets.push(i);
                targets_min_jumps.push(min_jump);
            }
        }
        if targets[targets.len() - 1] == 0 {
            targets_min_jumps[targets_min_jumps.len() - 1]
        } else {
            0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let nums = vec![7, 1, 5, 3, 6, 4];
        let result = Solution::jump(nums);
        assert_eq!(1, result);
    }

    #[test]
    fn test_2() {
        let nums = vec![2, 3, 1, 1, 4];
        let result = Solution::jump(nums);
        assert_eq!(2, result);
    }

    #[test]
    fn test_3() {
        let nums = vec![3, 2, 1, 0, 4];
        let result = Solution::jump(nums);
        assert_eq!(0, result);
    }

    #[test]
    fn test_4() {
        let nums = vec![7, 1, 5, 3, 6, 0];
        let result = Solution::jump(nums);
        assert_eq!(1, result);
    }

    #[test]
    fn test_5() {
        let nums = vec![0, 1, 5, 3, 6, 0];
        let result = Solution::jump(nums);
        assert_eq!(0, result);
    }

    #[test]
    fn test_6() {
        let nums = vec![0];
        let result = Solution::jump(nums);
        assert_eq!(0, result);
    }

    #[test]
    fn test_7() {
        let nums = vec![1, 0];
        let result = Solution::jump(nums);
        assert_eq!(1, result);
    }

    #[test]
    fn test_8() {
        let nums = vec![2, 0];
        let result = Solution::jump(nums);
        assert_eq!(1, result);
    }

    #[test]
    fn test_9() {
        let nums = vec![2, 0, 2, 0, 2, 0, 0];
        let result = Solution::jump(nums);
        assert_eq!(3, result);
    }

    #[test]
    fn test_10() {
        let nums = vec![2, 0, 2, 0, 1, 0, 0];
        let result = Solution::jump(nums);
        assert_eq!(0, result);
    }

    #[test]
    fn test_11() {
        let nums = vec![10, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0, 0];
        let result = Solution::jump(nums);
        assert_eq!(0, result);
    }
}
