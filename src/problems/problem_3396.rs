pub struct Solution;

use std::collections::HashSet;
impl Solution {
    pub fn minimum_operations(nums: Vec<i32>) -> i32 {
        let mut dubl_index = None;
        let mut set_nums = HashSet::new();
        for (i, x) in nums.iter().enumerate().rev() {
            if set_nums.contains(x) {
                dubl_index = Some(i);
                break;
            } else {
                set_nums.insert(x);
            }
        }
        if let Some(index) = dubl_index {
            println!("{}", index);
            let mut count_removes = (index + 3) / 3usize;
            return count_removes as i32;
        }
        return 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn problem_3396_ex1() {
        assert_eq!(
            Solution::minimum_operations(vec![1, 2, 3, 4, 2, 3, 3, 5, 7]),
            2
        );
    }

    #[test]
    fn problem_3396_ex2() {
        assert_eq!(Solution::minimum_operations(vec![4, 5, 6, 4, 4]), 2);
    }

    #[test]
    fn problem_3396_ex3() {
        assert_eq!(Solution::minimum_operations(vec![6, 7, 8, 9]), 0);
    }

    #[test]
    fn problem_3396_ex4() {
        assert_eq!(Solution::minimum_operations(vec![4, 4, 6, 5, 3]), 1);
    }
}
