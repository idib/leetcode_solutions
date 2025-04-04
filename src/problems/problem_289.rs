pub struct Solution;

impl Solution {
    pub fn game_of_life(board: &mut Vec<Vec<i32>>) {
        let las_step = board.clone();
        let m = las_step.len();
        let n = las_step[0].len();


        for i in 0..m {
            for j in 0..n {
                let mut live_neighbors = 0;

                for x in -1..=1 {
                    for y in -1..=1 {
                        if x == 0 && y == 0 {
                            continue;
                        }
                        let ni = i as isize + x;
                        let nj = j as isize + y;
                        if ni >= 0 && ni < m as isize && nj >= 0 && nj < n as isize {
                            live_neighbors += las_step[ni as usize][nj as usize];
                        }
                    }
                }

                if las_step[i][j] == 1 && (live_neighbors < 2 || live_neighbors > 3) {
                    board[i][j] = 0;
                } else if las_step[i][j] == 0 && live_neighbors == 3 {
                    board[i][j] = 1;
                }
            }
        }
        
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn problem_289_ex1() {
        let mut grid = vec![vec![0, 1, 0], vec![0, 0, 1], vec![1, 1, 1], vec![0, 0, 0]];
        let mut exept = vec![vec![0, 0, 0], vec![1, 0, 1], vec![0, 1, 1], vec![0, 1, 0]];

        Solution::game_of_life(grid.as_mut());
        assert_eq!(grid, exept)
    }

    #[test]
    fn problem_289_ex2() {
        let mut grid = vec![vec![1, 1], vec![1, 0]];
        let mut exept = vec![vec![1, 1], vec![1, 1]];

        Solution::game_of_life(grid.as_mut());
        assert_eq!(grid, exept)
    }
}
