pub struct Solution;

use std::collections::BinaryHeap;
impl Solution {
    pub fn max_kelements(nums: Vec<i32>, k: i32) -> i64 {
        let mut max_num: BinaryHeap<i32> = BinaryHeap::from(nums);

        let mut total_score: i64 = 0;
        for _ in 0..k {
            if let Some(num) = max_num.pop() {
                total_score += num as i64;
                let mut new_num = num / 3;
                if num % 3 != 0 {
                    new_num += 1
                }
                max_num.push(new_num);
            }
        }
        return total_score;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn problem_2530_ex1() {
        assert_eq!(Solution::max_kelements(vec![10, 10, 10, 10, 10], 5), 50);
    }
    #[test]
    fn problem_2530_ex2() {
        assert_eq!(Solution::max_kelements(vec![1, 10, 3, 3, 3], 3), 17);
    }
}
