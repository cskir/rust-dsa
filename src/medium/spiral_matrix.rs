pub struct Solution {}
impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let mut res: Vec<i32> = Vec::new();

        if matrix.is_empty() || matrix[0].is_empty() {
            return res;
        }

        for step in 0..((matrix.len().min(matrix[0].len()) + 1) / 2) {
            let row_start = step;
            let col_start = step;
            let row_end = matrix.len() - 1 - step;
            let col_end = matrix[0].len() - 1 - step;

            println!(
                "step: {}, row_start: {}, col_start: {}, row_end: {}, col_end: {}",
                step, row_start, col_start, row_end, col_end
            );
            if row_start > row_end || col_start > col_end {
                break;
            }

            if row_start == row_end {
                // single row
                matrix[row_start][col_start..=col_end]
                    .iter()
                    .for_each(|&x| res.push(x));
            } else if col_start == col_end {
                // single column
                matrix[row_start..=row_end]
                    .iter()
                    .for_each(|r| res.push(r[col_start]));
            } else {
                let mut circle =
                    Solution::get_circle(&matrix, row_start, col_start, row_end, col_end);
                res.append(&mut circle);
            }
        }

        res
    }

    fn get_circle(
        matrix: &Vec<Vec<i32>>,
        row_start: usize,
        col_start: usize,
        row_end: usize,
        col_end: usize,
    ) -> Vec<i32> {
        let mut res: Vec<i32> = Vec::new();

        matrix[row_start][col_start..=col_end]
            .iter()
            .for_each(|&x| res.push(x));

        matrix[row_start + 1..=row_end]
            .iter()
            .for_each(|r| res.push(r[col_end]));

        matrix[row_end][col_start..=col_end - 1]
            .iter()
            .rev()
            .for_each(|&x| res.push(x));

        matrix[row_start + 1..=row_end - 1]
            .iter()
            .rev()
            .for_each(|r| res.push(r[col_start]));

        res
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_circle_1() {
        let matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let result = Solution::get_circle(&matrix, 0, 0, 2, 2);
        assert_eq!(result, vec![1, 2, 3, 6, 9, 8, 7, 4]);
    }

    #[test]
    fn test_1() {
        let matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let result = Solution::spiral_order(matrix);
        assert_eq!(result, vec![1, 2, 3, 6, 9, 8, 7, 4, 5]);
    }

    #[test]
    fn test_circle_2() {
        let matrix = vec![vec![1, 2, 3, 4], vec![5, 6, 7, 8], vec![9, 10, 11, 12]];
        let result = Solution::get_circle(&matrix, 0, 0, 2, 3);
        assert_eq!(result, vec![1, 2, 3, 4, 8, 12, 11, 10, 9, 5]);
    }

    #[test]
    fn test_2() {
        let matrix = vec![vec![1, 2, 3, 4], vec![5, 6, 7, 8], vec![9, 10, 11, 12]];
        let result = Solution::spiral_order(matrix);
        assert_eq!(result, vec![1, 2, 3, 4, 8, 12, 11, 10, 9, 5, 6, 7]);
    }

    #[test]
    fn test_circle_3() {
        let matrix = vec![vec![1, 2], vec![3, 4]];
        let result = Solution::get_circle(&matrix, 0, 0, 1, 1);
        assert_eq!(result, vec![1, 2, 4, 3]);
    }

    #[test]
    fn test_3() {
        let matrix = vec![vec![1, 2], vec![3, 4]];
        let result = Solution::spiral_order(matrix);
        assert_eq!(result, vec![1, 2, 4, 3]);
    }

    #[test]
    fn test_circle_4() {
        let matrix = vec![
            vec![1, 2, 3, 4],
            vec![5, 6, 7, 8],
            vec![9, 10, 11, 12],
            vec![13, 14, 15, 16],
        ];
        let result = Solution::get_circle(&matrix, 0, 0, 3, 3);
        assert_eq!(result, vec![1, 2, 3, 4, 8, 12, 16, 15, 14, 13, 9, 5]);
    }

    #[test]
    fn test_4() {
        let matrix = vec![
            vec![1, 2, 3, 4],
            vec![5, 6, 7, 8],
            vec![9, 10, 11, 12],
            vec![13, 14, 15, 16],
        ];
        let result = Solution::spiral_order(matrix);
        assert_eq!(
            result,
            vec![1, 2, 3, 4, 8, 12, 16, 15, 14, 13, 9, 5, 6, 7, 11, 10]
        );
    }
}
