pub struct Solution;
// LeetCode Medium 73. Set Matrix Zeroes
impl Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let first_row_has_zero = matrix[0].iter().any(|&val| val == 0);
        let first_col_has_zero = matrix.iter().any(|row| row[0] == 0);

        // m,n > 0
        let n = matrix.len();
        let m = matrix[0].len();
        for i in 1..n {
            for j in 1..m {
                if matrix[i][j] == 0 {
                    matrix[i][0] = 0;
                    matrix[0][j] = 0;
                }
            }
        }

        for i in 1..n {
            for j in 1..m {
                if matrix[i][0] == 0 || matrix[0][j] == 0 {
                    matrix[i][j] = 0;
                }
            }
        }

        if first_row_has_zero {
            for j in 0..m {
                matrix[0][j] = 0;
            }
        }

        if first_col_has_zero {
            for i in 0..n {
                matrix[i][0] = 0;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_set_zeroes() {
        let mut matrix1 = vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]];
        Solution::set_zeroes(&mut matrix1);
        assert_eq!(matrix1, vec![vec![1, 0, 1], vec![0, 0, 0], vec![1, 0, 1],]);

        let mut matrix2 = vec![vec![0, 1, 2, 0], vec![3, 4, 5, 2], vec![1, 3, 1, 5]];
        Solution::set_zeroes(&mut matrix2);
        assert_eq!(
            matrix2,
            vec![vec![0, 0, 0, 0], vec![0, 4, 5, 0], vec![0, 3, 1, 0],]
        );
    }
}
