pub struct Solution {}

impl Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        // len > 0
        let mut j = 0;
        let mut sum = nums[0];
        let mut min = i32::MAX;

        for i in 0..nums.len() {
            println!("start  i {} j {} sum {}", i, j, sum);
            while sum < target && j < nums.len() - 1 {
                j += 1;
                sum += nums[j];
                println!("j {} sum {}", j, sum);
            }

            println!("check  sum {}", sum);
            if sum >= target {
                if j as i32 - i as i32 + 1 < min {
                    min = j as i32 - i as i32 + 1;

                    if min == 1 {
                        break;
                    }
                    println!("set min {}", min);
                }
            } else if j == nums.len() - 1 {
                println!("break");
                break;
            }

            sum -= nums[i];

            println!("set  sum {}", sum);
        }

        if min < i32::MAX { min } else { 0 }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1() {
        let nums = vec![1, 4, 4];
        assert_eq!(1, Solution::min_sub_array_len(0, nums));
    }

    #[test]
    fn test_2() {
        let nums = vec![1, 4, 4];
        assert_eq!(1, Solution::min_sub_array_len(4, nums));
    }

    #[test]
    fn test_3() {
        let nums = vec![1, 4, 4];
        assert_eq!(2, Solution::min_sub_array_len(5, nums));
    }

    #[test]
    fn test_4() {
        let nums = vec![1, 3, 4];
        assert_eq!(2, Solution::min_sub_array_len(5, nums));
    }

    #[test]
    fn test_5() {
        let nums = vec![1, 3, 4];
        assert_eq!(3, Solution::min_sub_array_len(8, nums));
    }

    #[test]
    fn test_6() {
        let nums = vec![12, 28, 83, 4, 25, 26, 25, 2, 25, 25, 25, 12];
        assert_eq!(8, Solution::min_sub_array_len(213, nums));
    }

    #[test]
    fn test_7() {
        let nums = vec![1, 1, 1, 1, 1, 1, 1, 1];
        assert_eq!(0, Solution::min_sub_array_len(11, nums));
    }
}
