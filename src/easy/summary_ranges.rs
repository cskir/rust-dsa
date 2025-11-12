pub struct Solution;
impl Solution {
    pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
        if nums.len() == 0 {
            return Vec::new();
        } else if nums.len() == 1 {
            return vec![nums[0].to_string()];
        }

        let mut result: Vec<String> = Vec::new();
        let mut start: usize = 0;

        for i in 1..nums.len() {
            if nums[i] != nums[i - 1] + 1 {
                if start == i - 1 {
                    // single number
                    result.push(nums[start].to_string());
                } else {
                    result.push(format!("{}->{}", nums[start], nums[i - 1]));
                }
                start = i;
            }
        }

        if start == nums.len() - 1 {
            result.push(nums[start].to_string());
        } else {
            result.push(format!("{}->{}", nums[start], nums[nums.len() - 1]));
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let nums = vec![0, 1, 2, 4, 5, 7];
        let result = Solution::summary_ranges(nums);
        let expected = vec!["0->2", "4->5", "7"];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_2() {
        let nums = vec![0, 2, 3, 4, 6, 8, 9];
        let result = Solution::summary_ranges(nums);
        let expected = vec!["0", "2->4", "6", "8->9"];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_3() {
        let nums: Vec<i32> = vec![];
        let result = Solution::summary_ranges(nums);
        let expected: Vec<String> = vec![];
        assert_eq!(result, expected);
    }
}
