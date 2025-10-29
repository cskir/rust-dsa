pub struct Solution {}
impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let mut n = matrix.len();
        for step in 0..n / 2 {
            Solution::handle_cw_circle(matrix, step, n);
            n -= 2;
        }
    }

    fn handle_cw_circle(matrix: &mut Vec<Vec<i32>>, offset: usize, n: usize) {
        let mut tmp: i32;
        for i in 0..(n - 1) {
            tmp = matrix[offset][offset + i];
            matrix[offset + 0][offset + i] = matrix[offset + n - 1 - i][offset + 0];
            matrix[offset + n - 1 - i][offset] = matrix[offset + n - 1][offset + n - 1 - i];
            matrix[offset + n - 1][offset + n - 1 - i] = matrix[offset + i][offset + n - 1];
            matrix[offset + i][offset + n - 1] = tmp;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_1() {
        let mut matrix = vec![vec![1]];
        Solution::rotate(&mut matrix);
        let expected = vec![vec![1]];
        assert_eq!(matrix, expected);
    }

    #[test]
    fn test_2() {
        let mut matrix = vec![vec![1, 2], vec![3, 4]];
        Solution::rotate(&mut matrix);
        let expected = vec![vec![3, 1], vec![4, 2]];
        assert_eq!(matrix, expected);
    }

    #[test]
    fn test_3() {
        let mut matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        Solution::rotate(&mut matrix);
        let expected = vec![vec![7, 4, 1], vec![8, 5, 2], vec![9, 6, 3]];
        assert_eq!(matrix, expected);
    }

    #[test]
    fn test_4() {
        let mut matrix = vec![
            vec![1, 2, 3, 4],
            vec![5, 6, 7, 8],
            vec![9, 10, 11, 12],
            vec![13, 14, 15, 16],
        ];
        Solution::rotate(&mut matrix);
        let expected = vec![
            vec![13, 9, 5, 1],
            vec![14, 10, 6, 2],
            vec![15, 11, 7, 3],
            vec![16, 12, 8, 4],
        ];
        assert_eq!(matrix, expected);
    }

    #[test]
    fn test_5() {
        let mut matrix = vec![
            vec![1, 2, 3, 4, 5, 6],
            vec![7, 8, 9, 10, 11, 12],
            vec![13, 14, 15, 16, 17, 18],
            vec![19, 20, 21, 22, 23, 24],
            vec![25, 26, 27, 28, 29, 30],
            vec![31, 32, 33, 34, 35, 36],
        ];
        Solution::rotate(&mut matrix);
        let expected = vec![
            vec![31, 25, 19, 13, 7, 1],
            vec![32, 26, 20, 14, 8, 2],
            vec![33, 27, 21, 15, 9, 3],
            vec![34, 28, 22, 16, 10, 4],
            vec![35, 29, 23, 17, 11, 5],
            vec![36, 30, 24, 18, 12, 6],
        ];
        assert_eq!(matrix, expected);
    }
}
