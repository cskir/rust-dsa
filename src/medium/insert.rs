pub struct Solution;
impl Solution {
    pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res: Vec<Vec<i32>> = Vec::new();
        if intervals.is_empty() {
            res.push(new_interval);
            return res;
        }
        if intervals.is_empty() {
            res.push(new_interval);
            return res;
        }
        let mut i: usize = 0;
        let mut used_new_interval = false;

        while i < intervals.len() {
            if intervals[i][1] < new_interval[0] {
                res.push(intervals[i].clone());
            } else if intervals[i][0] <= new_interval[0] && new_interval[1] <= intervals[i][1] {
                res.push(intervals[i].clone());
                used_new_interval = true;
            } else if new_interval[1] < intervals[i][0] {
                if !used_new_interval {
                    res.push(new_interval.clone());
                    used_new_interval = true;
                }
                res.push(intervals[i].clone());
            } else {
                let merge_start = new_interval[0].min(intervals[i][0]);
                while i < intervals.len() - 1 && intervals[i + 1][0] <= new_interval[1] {
                    i += 1;
                }

                res.push(vec![merge_start, new_interval[1].max(intervals[i][1])]);
                used_new_interval = true;
            }
            i += 1;
        }

        if !used_new_interval {
            res.push(new_interval.clone());
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let input = vec![vec![1, 3], vec![6, 9]];
        let to_insert = vec![2, 5];
        let result = Solution::insert(input, to_insert);
        let expected = vec![vec![1, 5], vec![6, 9]];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_2() {
        let input = vec![
            vec![1, 2],
            vec![3, 5],
            vec![6, 7],
            vec![8, 10],
            vec![12, 16],
        ];
        let to_insert = vec![4, 8];
        let result = Solution::insert(input, to_insert);
        let expected = vec![vec![1, 2], vec![3, 10], vec![12, 16]];
        assert_eq!(result, expected);
    }
    #[test]
    fn test_3() {
        let input = vec![];
        let to_insert = vec![5, 7];
        let result = Solution::insert(input, to_insert);
        let expected = vec![vec![5, 7]];
        assert_eq!(result, expected);
    }
    #[test]
    fn test_4() {
        let input = vec![vec![1, 5]];
        let to_insert = vec![2, 3];
        let result = Solution::insert(input, to_insert);
        let expected = vec![vec![1, 5]];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_5() {
        let input = vec![vec![1, 5]];
        let to_insert = vec![6, 8];
        let result = Solution::insert(input, to_insert);
        let expected = vec![vec![1, 5], vec![6, 8]];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_6() {
        let input = vec![
            vec![1, 2],
            vec![4, 6],
            vec![8, 10],
            vec![11, 13],
            vec![14, 16],
        ];
        let to_insert = vec![7, 11];
        let result = Solution::insert(input, to_insert);
        let expected = vec![vec![1, 2], vec![4, 6], vec![7, 13], vec![14, 16]];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_7() {
        let input = vec![vec![1, 2], vec![3, 6], vec![8, 10]];
        let to_insert = vec![2, 9];
        let result = Solution::insert(input, to_insert);
        let expected = vec![vec![1, 10]];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_8() {
        let input = vec![vec![2, 5]];
        let to_insert = vec![0, 1];
        let result = Solution::insert(input, to_insert);
        let expected = vec![vec![0, 1], vec![2, 5]];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_9() {
        let input = vec![vec![0, 5], vec![8, 10]];
        let to_insert = vec![3, 4];
        let result = Solution::insert(input, to_insert);
        let expected = vec![vec![0, 5], vec![8, 10]];
        assert_eq!(result, expected);
    }
}
