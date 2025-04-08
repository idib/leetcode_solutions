pub struct Solution;

impl Solution {
    pub fn repeated_n_times(mut nums: Vec<i32>) -> i32 {
        let n = nums.len() / 2;
        nums.sort();
        for i in 0..n+1 {
            if nums[i] == nums[i + 1] {
                return nums[i];
            }
        }
        return 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn problem_961_ex1() {
        assert_eq!(Solution::repeated_n_times(vec![1, 2, 3, 3]), 3);
    }

    #[test]
    fn problem_961_ex2() {
        assert_eq!(Solution::repeated_n_times(vec![2, 1, 2, 5, 3, 2]), 2);
    }

    #[test]
    fn problem_961_ex3() {
        assert_eq!(Solution::repeated_n_times(vec![5, 1, 5, 2, 5, 3, 5, 4]), 5);
    }
}
