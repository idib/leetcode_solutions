pub struct Solution;

impl Solution {
    pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {

        let mut carry: Option<i32> = Some(1);

        let n = digits.len();
        for i in 0..n {
            if let Some(c) = carry {
                let d = digits[n - i - 1] + c;
                digits[n - i - 1] = d % 10;
                if d / 10 == 1 {
                    carry = Some(1);
                } else {
                    carry = None;
                }
            }
        }
        if let Some(c) = carry {
            digits.insert(0, c)
        }

        return digits;
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn problem_66_ex1() {
        assert_eq!(Solution::plus_one(vec![1, 2, 3]), vec![1, 2, 4]);
    }

    #[test]
    fn problem_66_ex2() {
        assert_eq!(Solution::plus_one(vec![4, 3, 2, 1]), vec![4, 3, 2, 2]);
    }
    #[test]
    fn problem_66_ex3() {
        assert_eq!(Solution::plus_one(vec![9]), vec![1, 0]);
    }
}
