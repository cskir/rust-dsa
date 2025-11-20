use std::cmp::min;

// Leet Code 452. Minimum Number of Arrows to Burst Balloons
pub struct Solution {}
impl Solution {
    pub fn find_min_arrow_shots(points: Vec<Vec<i32>>) -> i32 {
        if points.len() == 0 {
            return 0;
        }

        let mut points = points;
        points.sort_by_key(|k| k[0]);

        println!("{:?}", points);
        let mut intersect = points[0].clone();
        let mut res = 0;
        for i in 1..points.len() {
            if points[i][0] <= intersect[1] {
                intersect = vec![points[i][0], min(intersect[1], points[i][1])];
            } else {
                intersect = points[i].clone();
                res += 1;
            }
            println!("{:?} {:?} res: {} ", intersect, points[i], res);
        }

        res + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let input = vec![vec![10, 16], vec![2, 8], vec![1, 6], vec![7, 12]];
        let result = Solution::find_min_arrow_shots(input);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_2() {
        let input = vec![vec![1, 2], vec![3, 4], vec![5, 6], vec![7, 8]];
        let result = Solution::find_min_arrow_shots(input);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_3() {
        let input = vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5]];
        let result = Solution::find_min_arrow_shots(input);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_4() {
        let input = vec![vec![-10, -5], vec![-7, 0], vec![2, 5], vec![7, 10]];
        let result = Solution::find_min_arrow_shots(input);
        assert_eq!(result, 3);
    }

    #[test]
    fn test_5() {
        //[[3,9],[7,12],[3,8],[6,8],[9,10],[2,9],[0,9],[3,9],[0,6],[2,8]]
        let input = vec![
            vec![3, 9],
            vec![7, 12],
            vec![3, 8],
            vec![6, 8],
            vec![9, 10],
            vec![2, 9],
            vec![0, 9],
            vec![3, 9],
            vec![0, 6],
            vec![2, 8],
        ];
        let result = Solution::find_min_arrow_shots(input);
        assert_eq!(result, 2);
    }
}
