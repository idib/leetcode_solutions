pub struct Solution;

impl Solution {
    pub fn check_zero_ones(s: String) -> bool {
        let mut max_ones = 0;
        let mut max_zeros = 0;
        let mut cur_ones = 0;
        let mut cur_zeros = 0;

        let mut prev: Option<char> = None;
        for x in s.chars() {
            if let Some(p) = prev {
                if x == '1' && p == '0' {
                    if max_zeros < cur_zeros {
                        max_zeros = cur_zeros;
                    }
                    cur_zeros = 0;
                }
                if x == '0' && p == '1' {
                    if max_ones < cur_ones {
                        max_ones = cur_ones
                    }
                    cur_ones = 0;
                }
            }
            if x == '1' {
                cur_ones += 1;
            }
            if x == '0' {
                cur_zeros += 1;
            }
            prev = Some(x)
        }

        if max_zeros < cur_zeros {
            max_zeros = cur_zeros;
        }
        if max_ones < cur_ones {
            max_ones = cur_ones
        }
        return max_ones > max_zeros;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn problem_1869_ex1() {
        assert_eq!(Solution::check_zero_ones(String::from("1101")), true);
    }

    #[test]
    fn problem_1869_ex2() {
        assert_eq!(Solution::check_zero_ones(String::from("111000")), false);
    }

    #[test]
    fn problem_1869_ex3() {
        assert_eq!(Solution::check_zero_ones(String::from("110100010")), false);
    }
}
