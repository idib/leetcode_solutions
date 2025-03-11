pub struct Solution;

impl Solution {
    pub fn make_sub_k_sum_equal(arr: Vec<i32>, k: i32) -> i64 {
        return 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn problem_2607_ex1() {
        assert_eq!(Solution::make_sub_k_sum_equal(vec![1, 4, 1, 3], 2), 1);
    }

    #[test]
    fn problem_2607_ex2() {
        assert_eq!(Solution::make_sub_k_sum_equal(vec![2, 5, 5, 7], 3), 5);
    }

    #[test]
    fn problem_2607_ex3() {
        assert_eq!(Solution::make_sub_k_sum_equal(vec![9], 1), 0);
    }

    #[test]
    fn problem_2607_ex4() {
        assert_eq!(Solution::make_sub_k_sum_equal(vec![1, 10], 1), 9);
    }

    #[test]
    fn problem_2607_ex5() {
        assert_eq!(Solution::make_sub_k_sum_equal(vec![4, 6], 2), 2);
    }

    #[test]
    fn problem_2607_ex6() {
        assert_eq!(Solution::make_sub_k_sum_equal(vec![2, 10, 9], 1), 8);
    }
}
