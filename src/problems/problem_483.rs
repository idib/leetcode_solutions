pub struct Solution;

impl Solution {
    pub fn smallest_good_base(n: String) -> String {
        let num: u64 = n.parse::<u64>().unwrap();
        let max_len_ones = (num as f64).log2() as u32;
        for m in (2..=max_len_ones).rev() {
            let k = (num as f64).powf(1.0 / (m as f64)) as u64;
            if k < 2 {
                continue;
            }

            let mut sum = 1u64;
            let mut cur = 1u64;
            for _ in 0..m {
                if let Some(next) = cur.checked_mul(k) {
                    cur = next;
                } else {
                    cur = num;
                }
                sum = sum.saturating_add(cur);
                if sum > num {
                    break;
                }
            }
            if sum == num {
                return k.to_string();
            }
        }

        (num - 1).to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn problem_282_ex1() {
        assert_eq!(
            Solution::smallest_good_base("13".to_string()),
            "3".to_string()
        )
    }

    #[test]
    fn problem_282_ex2() {
        assert_eq!(
            Solution::smallest_good_base("4681".to_string()),
            "8".to_string()
        )
    }

    #[test]
    fn problem_282_ex3() {
        assert_eq!(
            Solution::smallest_good_base("1000000000000000000".to_string()),
            "999999999999999999".to_string()
        )
    }
}
