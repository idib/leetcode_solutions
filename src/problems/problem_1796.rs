pub struct Solution;

impl Solution {
    pub fn second_highest(s: String) -> i32 {
        let mut digits = vec![0; 10];

        for x in s.chars() {
            if x.is_numeric() {
                digits[(x as usize - '0' as usize)] += 1;
            }
        }

        println!("{:?}", digits);

        let mut found_highest = false;
        for i in 0..10 {
            if digits[9 - i] > 0 {
                if found_highest {
                    return 9 - i as i32;
                } else {
                    found_highest = true;
                }
            }
        }

        return -1;
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn problem_1796_ex1() {
        assert_eq!(Solution::second_highest(String::from("dfa12321afd")), 2);
    }

    #[test]
    fn problem_1796_ex2() {
        assert_eq!(Solution::second_highest(String::from("abc1111")), -1);
    }

    #[test]
    fn problem_1796_ex3() {
        assert_eq!(Solution::second_highest(String::from("ck077")), 0);
    }
}
