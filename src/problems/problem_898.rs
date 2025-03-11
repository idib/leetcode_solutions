pub struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn subarray_bitwise_o_rs(arr: Vec<i32>) -> i32 {
        let n = arr.len();

        let mut uniq_nums: HashSet<i32> = HashSet::new();
        let mut cur = HashSet::new();

        for x in arr {
            let mut next = HashSet::new();
            for y in cur.iter() {
                next.insert(y | x);
            }
            next.insert(x);
            uniq_nums.extend(&next);
            cur = next;
        }

        return uniq_nums.len() as i32;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn problem_898_ex1() {
        assert_eq!(Solution::subarray_bitwise_o_rs(vec![0]), 1);
    }

    #[test]
    fn problem_898_ex2() {
        assert_eq!(Solution::subarray_bitwise_o_rs(vec![1, 1, 2]), 3);
    }

    #[test]
    fn problem_898_ex3() {
        assert_eq!(Solution::subarray_bitwise_o_rs(vec![1, 2, 4]), 6);
    }
}
