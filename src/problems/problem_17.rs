pub struct Solution;

const DICTIONARY: [&str; 8] = ["abc", "def", "ghi", "jkl", "mno", "pqrs", "tuv", "wxyz"];

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        let n = digits.len();
        let mut result = Vec::<String>::new();

        for (i, x) in digits.chars().enumerate() {
            let index = x as usize - '2' as usize;
            let mut next = Vec::new();
            for new_char in DICTIONARY[index].chars() {
                if i == 0 {
                    next.push(new_char.to_string())
                } else {
                    for part_str in result.iter() {
                        let mut new_str = String::new();
                        new_str.push_str(part_str);
                        new_str.push(new_char);
                        next.push(new_str);
                    }
                }
            }
            result = next
        }
        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn problem_17_ex1() {
        let mut got = Solution::letter_combinations("23".to_string());
        got.sort();
        let mut want = vec!["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"];
        want.sort();

        assert_eq!(got, want)
    }

    #[test]
    fn problem_17_ex2() {
        assert_eq!(
            Solution::letter_combinations("".to_string()),
            Vec::<String>::new()
        );
    }

    #[test]
    fn problem_17_ex3() {
        assert_eq!(
            Solution::letter_combinations("2".to_string()),
            vec!["a", "b", "c"]
        );
    }
}
