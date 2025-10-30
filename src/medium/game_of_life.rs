pub struct Solution {}
impl Solution {
    pub fn game_of_life(board: &mut Vec<Vec<i32>>) {
        let mut changes: Vec<(usize, usize)> = vec![];
        for i in 0..board.len() {
            for j in 0..board[0].len() {
                let mut live_neighbors_sum = 0;
                for x in -1..=1 {
                    for y in -1..=1 {
                        if x == 0 && y == 0 {
                            continue;
                        }
                        let ni = i as isize + x;
                        let nj = j as isize + y;
                        if ni >= 0
                            && ni < board.len() as isize
                            && nj >= 0
                            && nj < board[0].len() as isize
                            && board[ni as usize][nj as usize] == 1
                        {
                            live_neighbors_sum += 1;
                        }
                    }
                }

                if (board[i][j] == 1 && (live_neighbors_sum < 2 || live_neighbors_sum > 3))
                    || (board[i][j] == 0 && live_neighbors_sum == 3)
                {
                    changes.push((i, j));
                }
            }
        }

        for (i, j) in changes {
            board[i][j] ^= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_game_of_life() {
        let mut board = vec![vec![0, 1, 0], vec![0, 0, 1], vec![1, 1, 1], vec![0, 0, 0]];
        super::Solution::game_of_life(&mut board);
        assert_eq!(
            board,
            vec![vec![0, 0, 0], vec![1, 0, 1], vec![0, 1, 1], vec![0, 1, 0],]
        );
    }

    #[test]
    fn test_game_of_life2() {
        let mut board = vec![vec![1, 1], vec![1, 0]];
        super::Solution::game_of_life(&mut board);
        assert_eq!(board, vec![vec![1, 1], vec![1, 1],]);
    }
}
