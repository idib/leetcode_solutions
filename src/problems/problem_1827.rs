pub struct Solution;

impl Solution {
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        let mut sum = 0;
        let mut prev = nums[0];
        for i in 1..nums.len() {
            if prev >= nums[i] {
                sum += prev - nums[i] + 1;
                prev += 1;
            }else{
                prev = nums[i]
            }
        }
        return sum;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn problem_1827_ex1() {
        assert_eq!(Solution::min_operations(vec![1, 1, 1]), 3);
    }

    #[test]
    fn problem_1827_ex2() {
        assert_eq!(Solution::min_operations(vec![1, 5, 2, 4, 1]), 14);
    }

    #[test]
    fn problem_1827_ex3() {
        assert_eq!(Solution::min_operations(vec![8]), 0);
    }

    #[test]
    fn problem_1827_ex4() {
        assert_eq!(Solution::min_operations(vec![1, 8]), 0);
    }
}
