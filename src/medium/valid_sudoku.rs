use std::collections::{HashMap, HashSet};

pub struct Solution {}
impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let valid_set = HashSet::from(['1', '2', '3', '4', '5', '6', '7', '8', '9']);

        let mut row_set: HashMap<usize, HashSet<char>> = HashMap::new();
        let mut col_set: HashMap<usize, HashSet<char>> = HashMap::new();
        let mut box_set: HashMap<usize, HashSet<char>> = HashMap::new();
        for i in 0..9 {
            row_set.insert(i, valid_set.clone());
            col_set.insert(i, valid_set.clone());
            box_set.insert(i, valid_set.clone());
        }

        for row in 0..9 {
            for col in 0..9 {
                if board[row][col] == '.' {
                    continue;
                }
                if !row_set[&row].contains(&board[row][col])
                    || !col_set[&col].contains(&board[row][col])
                    || !box_set[&Solution::map_to_box(row, col)].contains(&board[row][col])
                {
                    return false;
                }

                row_set.get_mut(&row).unwrap().remove(&board[row][col]);

                col_set.get_mut(&col).unwrap().remove(&board[row][col]);

                box_set
                    .get_mut(&Solution::map_to_box(row, col))
                    .unwrap()
                    .remove(&board[row][col]);
            }
        }

        true
    }

    fn map_to_box(row: usize, col: usize) -> usize {
        (row / 3) * 3 + (col / 3)
    }
}
