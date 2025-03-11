use std::collections::BinaryHeap;

pub struct Solution;

impl Solution {
    pub fn delete_greatest_value(mut grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();

        for i in 0..m {
            grid[i].sort();
        }

        let mut sum = 0;
        for i in 0..n {
            let mut max_for_col = grid[0][i];
            for j in 1..m {
                max_for_col = grid[j][i].max(max_for_col)
            }
            sum += max_for_col;
        }

        return sum;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn problem_2500_ex1() {
        assert_eq!(
            Solution::delete_greatest_value(vec![
                vec![1, 2, 4], // 1 line
                vec![3, 3, 1]  // 2 line
            ]),
            8
        );
    }
    #[test]
    fn problem_2500_ex2() {
        assert_eq!(
            Solution::delete_greatest_value(vec![
                vec![10], // 1 line
            ]),
            10
        );
    }
}
