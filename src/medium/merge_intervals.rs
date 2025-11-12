pub struct Solution;
impl Solution {
    pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        if intervals.len() <= 1 {
            return intervals;
        }
        let mut intervals = intervals;

        intervals.sort_by_key(|k| k[0]);

        let mut result: Vec<Vec<i32>> = Vec::new();
        let mut merged = intervals[0].clone();
        for i in 1..intervals.len() {
            if merged[1] >= intervals[i][0] && merged[0] <= intervals[i][1] {
                merged[1] = merged[1].max(intervals[i][1]);
            } else {
                result.push(merged);
                merged = intervals[i].clone();
            }
        }
        result.push(merged);

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let input = vec![vec![1, 3], vec![2, 4], vec![5, 7], vec![6, 8]];
        let result = Solution::merge(input);
        let expected = vec![vec![1, 4], vec![5, 8]];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_2() {
        let input = vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]];
        let result = Solution::merge(input);
        let expected = vec![vec![1, 6], vec![8, 10], vec![15, 18]];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_3() {
        let input = vec![vec![1, 4], vec![4, 5]];
        let result = Solution::merge(input);
        let expected = vec![vec![1, 5]];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_4() {
        let input = vec![vec![1, 4], vec![4, 7]];
        let result = Solution::merge(input);
        let expected = vec![vec![1, 7]];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_5() {
        let input = vec![vec![4, 7], vec![1, 4]];
        let result = Solution::merge(input);
        let expected = vec![vec![1, 7]];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_6() {
        let input = vec![vec![1, 1], vec![2, 2]];
        let result = Solution::merge(input);
        let expected = vec![vec![1, 1], vec![2, 2]];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_7() {
        let input = vec![vec![2, 3], vec![4, 5], vec![6, 7], vec![8, 9], vec![1, 10]];
        let result = Solution::merge(input);
        let expected = vec![vec![1, 10]];
        assert_eq!(result, expected);
    }
}
