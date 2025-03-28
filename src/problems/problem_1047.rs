pub struct Solution;

impl Solution {
    pub fn remove_duplicates(s: String) -> String {
        let mut stack: String = String::new();
        for x in s.chars() {
            if let Some(top) = stack.pop() {
                if x != top {
                    stack.push(top);
                    stack.push(x);
                }
            } else {
                stack.push(x);
            }
        }
        return stack;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn problem_53_ex1() {
        assert_eq!(
            Solution::remove_duplicates("abbaca".to_string()),
            "ca".to_string()
        );
    }

    #[test]
    fn problem_53_ex2() {
        assert_eq!(
            Solution::remove_duplicates("azxxzy".to_string()),
            "ay".to_string()
        );
    }
}
