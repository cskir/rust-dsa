pub struct Solution {}
impl Solution {
    pub fn h_index(citations: Vec<i32>) -> i32 {
        // len > 0
        if citations.len() == 1 {
            return if citations[0] == 0 { 0 } else { 1 };
        }

        let mut citations = citations;
        citations.sort();
        citations.reverse();
        let mut result = 0;
        for i in 0..citations.len() {
            if citations[i] >= i as i32 + 1 {
                result += 1;
            } else {
                break;
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let nums = vec![3, 0, 6, 1, 5];
        let result = Solution::h_index(nums);
        assert_eq!(3, result);
    }

    #[test]
    fn test_2() {
        let nums = vec![1, 3, 1];
        let result = Solution::h_index(nums);
        assert_eq!(1, result);
    }

    #[test]
    fn test_3() {
        let nums = vec![3, 2, 1, 0, 4];
        let result = Solution::h_index(nums);
        assert_eq!(2, result);
    }

    #[test]
    fn test_4() {
        let nums = vec![7, 1, 5, 3, 6, 0];
        let result = Solution::h_index(nums);
        assert_eq!(3, result);
    }

    #[test]
    fn test_5() {
        let nums = vec![0, 1, 5, 3, 6, 0];
        let result = Solution::h_index(nums);
        assert_eq!(3, result);
    }

    #[test]
    fn test_6() {
        let nums = vec![0];
        let result = Solution::h_index(nums);
        assert_eq!(0, result);
    }

    #[test]
    fn test_7() {
        let nums = vec![1, 0];
        let result = Solution::h_index(nums);
        assert_eq!(1, result);
    }

    #[test]
    fn test_8() {
        let nums = vec![2, 0];
        let result = Solution::h_index(nums);
        assert_eq!(1, result);
    }

    #[test]
    fn test_9() {
        let nums = vec![2, 0, 2, 0, 2, 0, 0];
        let result = Solution::h_index(nums);
        assert_eq!(2, result);
    }

    #[test]
    fn test_10() {
        let nums = vec![2, 0, 2, 0, 1, 0, 0];
        let result = Solution::h_index(nums);
        assert_eq!(2, result);
    }

    #[test]
    fn test_11() {
        let nums = vec![10, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0, 0];
        let result = Solution::h_index(nums);
        assert_eq!(6, result);
    }

    #[test]
    fn test_12() {
        let nums = vec![7, 1, 5, 3, 6, 4];
        let result = Solution::h_index(nums);
        assert_eq!(4, result);
    }

    #[test]
    fn test_13() {
        let nums = vec![2, 3, 1, 1, 4];
        let result = Solution::h_index(nums);
        assert_eq!(2, result);
    }

    #[test]
    fn test_14() {
        let nums = vec![1];
        let result = Solution::h_index(nums);
        assert_eq!(1, result);
    }

    #[test]
    fn test_15() {
        let nums = vec![2];
        let result = Solution::h_index(nums);
        assert_eq!(1, result);
    }

    #[test]
    fn test_16() {
        let nums = vec![3, 4];
        let result = Solution::h_index(nums);
        assert_eq!(2, result);
    }
}
