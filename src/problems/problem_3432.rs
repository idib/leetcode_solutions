pub struct Solution;

impl Solution {
    pub fn count_partitions(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n < 2 {
            return 0;
        }

        let mut count_odd = vec![0; n];
        count_odd[0] = nums[0] % 2;
        for i in 1..n {
            count_odd[i] = count_odd[i - 1] + nums[i] % 2
        }

        let mut res: i32 = n as i32 - 1;
        let count_all_odd = count_odd[n - 1];
        for i in 0..n - 1 {
            let count_in_left = count_odd[i];
            let count_in_right = count_all_odd - count_odd[i];

            res -= (count_in_left - count_in_right).abs() % 2;
        }
        return res;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn problem_3432_ex1() {
        assert_eq!(Solution::count_partitions(vec![10, 10, 3, 7, 6]), 4);
    }

    #[test]
    fn problem_3432_ex2() {
        assert_eq!(Solution::count_partitions(vec![1, 2, 2]), 0);
    }

    #[test]
    fn problem_3432_ex3() {
        assert_eq!(Solution::count_partitions(vec![2, 4, 6, 8]), 3);
    }

    #[test]
    fn problem_3432_ex4() {
        assert_eq!(Solution::count_partitions(vec![1, 2]), 0);
    }

    #[test]
    fn problem_3432_ex5() {
        assert_eq!(Solution::count_partitions(vec![1, 2, 1]), 2);
    }

    #[test]
    fn problem_3432_ex6() {
        assert_eq!(Solution::count_partitions(vec![1, 2, 1, 1]), 0);
    }
}
