pub struct Solution {}
impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut nums = nums;
        nums.sort();

        let mut res: Vec<Vec<i32>> = Vec::new();

        let mut sum: i32;
        let mut j: usize;
        let mut k: usize;
        let mut tmp: i32;
        for i in 0..nums.len() - 2 {
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }
            if nums[i] > 0 {
                break;
            }

            j = i + 1;
            k = nums.len() - 1;
            sum = 0 - nums[i];
            while j < k {
                tmp = nums[j] + nums[k];
                if tmp == sum {
                    res.push(vec![nums[i], nums[j], nums[k]]);

                    j += 1;
                    k -= 1;
                    while j < k && nums[j] == nums[j - 1] {
                        j += 1;
                    }
                    while j < k && nums[k] == nums[k + 1] {
                        k -= 1;
                    }
                } else if tmp < sum {
                    j += 1;
                } else {
                    k -= 1;
                }
            }
        }

        res
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1() {
        let nums = vec![0, 0, 0];
        assert_eq!(vec![vec![0, 0, 0]], Solution::three_sum(nums));
    }

    #[test]
    fn test_2() {
        let nums = vec![0, 1, 1];
        let expected: Vec<Vec<i32>> = vec![];
        assert_eq!(expected, Solution::three_sum(nums));
    }

    #[test]
    fn test_3() {
        let nums = vec![-1, 0, 1, 2, -1, -4];
        assert_eq!(
            vec![vec![-1, -1, 2], vec![-1, 0, 1]],
            Solution::three_sum(nums)
        );
    }

    #[test]
    fn test_4() {
        let nums = vec![-2, 0, 1, 1, 2];
        assert_eq!(
            vec![vec![-2, 0, 2], vec![-2, 1, 1]],
            Solution::three_sum(nums)
        );
    }

    #[test]
    fn test_5() {
        let nums = vec![-4, -2, -2, -2, 0, 1, 2, 2, 2, 3, 3, 4, 4, 6, 6];
        assert_eq!(
            vec![
                vec![-4, -2, 6],
                vec![-4, 0, 4],
                vec![-4, 1, 3],
                vec![-4, 2, 2],
                vec![-2, -2, 4],
                vec![-2, 0, 2]
            ],
            Solution::three_sum(nums)
        );
    }
}
