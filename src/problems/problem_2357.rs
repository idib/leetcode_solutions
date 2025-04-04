pub struct Solution;

impl Solution {
    pub fn minimum_operations(mut nums: Vec<i32>) -> i32 {
        nums.sort();
        let mut res = 0;
        let mut sum_subtract = 0;
        for x in nums {
            if x > 0 && x > sum_subtract {
                sum_subtract += x - sum_subtract;
                res += 1;
            }
        }
        return res;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn problem_2357_ex1() {
        assert_eq!(Solution::minimum_operations(vec![1, 5, 0, 3, 5]), 3);
    }

    #[test]
    fn problem_2357_ex2() {
        assert_eq!(Solution::minimum_operations(vec![0]), 0);
    }

    #[test]
    fn problem_2357_ex3() {
        assert_eq!(Solution::minimum_operations(vec![0, 1, 2, 3]), 3);
    }
}
