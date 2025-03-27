pub struct Solution;

const ZERO_CODE: i32 = '0' as i32;
impl Solution {
    pub fn add_operators(num: String, target: i32) -> Vec<String> {
        let n = num.len();
        if n < 1 {
            return vec![];
        }
        let mut nums: Vec<i32> = num.chars().map(|x| x as i32 - ZERO_CODE).collect();

        let res = Self::add_operators_recursively(&mut nums[..], target).unwrap_or(vec![]);

        return res;
    }

    fn add_operators_recursively(nums: &mut [i32], target: i32) -> Option<Vec<String>> {
        let n = nums.len();
        if n == 1 {
            if target == nums[0] {
                return Some(vec![nums[0].to_string()]);
            } else {
                return None;
            }
        }

        let mut res: Vec<String> = vec![];

        let first = nums[0];
        let second = nums[1];

        let option_res = Self::add_operators_recursively(&mut nums[1..], target - first);
        match option_res {
            None => {}
            Some(mut found) => {
                for i in 0..found.len() {
                    found[i].insert(0, (first + ZERO_CODE) as u8 as char);
                    found[i].insert(1, '+');
                }
                res.extend(found);
            }
        }

        let option_res = Self::add_operators_recursively(&mut nums[1..], target + first);
        match option_res {
            None => {}
            Some(mut found) => {
                for i in 0..found.len() {
                    found[i].insert(0, (first + ZERO_CODE) as u8 as char);
                    found[i].insert(1, '-');
                }
                res.extend(found);
            }
        }

        if n > 1 {
            nums[1] *= first;
            let option_res = Self::add_operators_recursively(&mut nums[1..], target);
            match option_res {
                None => {}
                Some(mut found) => {
                    for i in 0..found.len() {
                        found[i].insert(0, (first + ZERO_CODE) as u8 as char);
                        found[i].insert(1, '*');
                    }
                    res.extend(found);
                }
            }
            nums[1] = second;
            if first != 0 {
                nums[1] = first * 10 + second;
                let option_res = Self::add_operators_recursively(&mut nums[1..], target);
                match option_res {
                    None => {}
                    Some(mut found) => {
                        for i in 0..found.len() {
                            found[i].insert(0, (first + ZERO_CODE) as u8 as char);
                        }
                        res.extend(found);
                    }
                }
                nums[1] = second;
            }
        }
        return Some(res);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn problem_282_ex1() {
        let mut a = HashSet::new();
        a.extend(Solution::add_operators("123".to_string(), 6));

        println!("123");
        println!("{:?}", a);
        assert_eq!(a.len(), 2);
        assert!(a.contains("1*2*3"));
        assert!(a.contains("1+2+3"));
    }

    #[test]
    fn problem_282_ex2() {
        let mut a = HashSet::new();
        a.extend(Solution::add_operators("232".to_string(), 8));

        println!("{:?}", a);
        assert_eq!(a.len(), 2);
        assert!(a.contains("2*3+2"));
        assert!(a.contains("2+3*2"));
    }

    #[test]
    fn problem_282_ex3() {
        assert_eq!(
            Solution::add_operators("3456237490".to_string(), 9191),
            vec![""; 0]
        );
    }

    #[test]
    fn problem_282_ex4() {
        let mut a = HashSet::new();
        a.extend(Solution::add_operators("105".to_string(), 5));

        println!("{:?}", a);
        assert_eq!(a.len(), 2);
        assert!(a.contains("1*0+5"));
        assert!(a.contains("10-5"));
    }
}
