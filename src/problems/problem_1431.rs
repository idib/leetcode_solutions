pub struct Solution;

impl Solution {
    pub fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
        let max = candies.iter().max().unwrap();
        let mut res = vec![false; candies.len()];
        for (i,v) in candies.iter().enumerate() {
            res[i] = v + extra_candies >= *max;
        }
        return res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn problem_1431_ex1() {
        assert_eq!(
            Solution::kids_with_candies(vec![2, 3, 5, 1, 3], 3),
            vec![true, true, true, false, true]
        );
    }

    #[test]
    fn problem_1431_ex2() {
        assert_eq!(
            Solution::kids_with_candies(vec![4, 2, 1, 1, 2], 1),
            vec![true, false, false, false, false]
        );
    }

    #[test]
    fn problem_1431_ex3() {
        assert_eq!(
            Solution::kids_with_candies(vec![12, 1, 12], 10),
            vec![true, false, true]
        );
    }
}
