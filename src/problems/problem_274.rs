pub struct Solution;

use std::cmp::min;
impl Solution {
    pub fn h_index(mut citations: Vec<i32>) -> i32 {
        citations.sort();
        let mut max_h = 0;
        for (i, v) in citations.iter().rev().enumerate() {
            max_h = max_h.max(min(i as i32 + 1, *v))
        }
        return max_h;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn problem_274_ex1() {
        assert_eq!(Solution::h_index(vec![3, 0, 6, 1, 5]), 3);
    }

    #[test]
    fn problem_274_ex2() {
        assert_eq!(Solution::h_index(vec![1, 3, 1]), 1);
    }

    #[test]
    fn problem_274_ex3() {
        assert_eq!(Solution::h_index(vec![1, 3, 1, 2]), 2);
    }

    #[test]
    fn problem_274_ex4() {
        assert_eq!(Solution::h_index(vec![0, 1, 2, 3, 4, 5, 6]), 3);
    }

    #[test]
    fn problem_274_ex5() {
        assert_eq!(Solution::h_index(vec![0, 1, 2, 3, 4, 5, 6, 1000]), 4);
    }

    #[test]
    fn problem_274_ex6() {
        assert_eq!(Solution::h_index(vec![1000]), 1);
    }

    #[test]
    fn problem_274_ex7() {
        assert_eq!(
            Solution::h_index(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]),
            1
        );
    }

    #[test]
    fn problem_274_ex8() {
        assert_eq!(Solution::h_index(vec![12, 11, 10, 9, 8, 1]), 5);
    }
}
