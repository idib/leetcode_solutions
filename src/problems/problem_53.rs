pub struct Solution;

use std::cmp::max;
impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut max_current = nums[0];
        let mut max_global = nums[0];

        for &num in nums.iter().skip(1) {
            max_current = max(num, max_current + num);
            max_global = max_global.max(max_current);
        }
        return max_global;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn problem_53_ex1() {
        assert_eq!(
            Solution::max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]),
            6
        );
    }

    #[test]
    fn problem_53_ex2() {
        assert_eq!(Solution::max_sub_array(vec![1]), 1);
    }

    #[test]
    fn problem_53_ex3() {
        assert_eq!(Solution::max_sub_array(vec![5, 4, -1, 7, 8]), 23);
    }

    #[test]
    fn problem_53_ex4() {
        assert_eq!(Solution::max_sub_array(vec![-1]), -1);
    }
}
