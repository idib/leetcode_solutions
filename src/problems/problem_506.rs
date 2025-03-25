pub struct Solution;

use std::collections::BinaryHeap;
use std::string::ToString;
const TOP_3_RANK_NAME: [&str; 3] = ["Gold Medal", "Silver Medal", "Bronze Medal"];
impl Solution {
    pub fn find_relative_ranks(score: Vec<i32>) -> Vec<String> {
        let n = score.len();
        let mut top_ranks = BinaryHeap::new();
        let mut ranks_names: Vec<String> = vec![String::new(); n];

        for (i, x) in score.iter().enumerate() {
            top_ranks.push((x, i));
        }

        for top_index in 0..n {
            let (_, index) = top_ranks.pop().unwrap();
            if top_index < 3 {
                ranks_names[index] = TOP_3_RANK_NAME[top_index].to_string();
            }else {
                ranks_names[index] = (top_index+1).to_string();
            }
        }

        return ranks_names;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn problem_506_ex1() {
        assert_eq!(
            Solution::find_relative_ranks(vec![5, 4, 3, 2, 1]),
            ["Gold Medal", "Silver Medal", "Bronze Medal", "4", "5"]
        );
    }

    #[test]
    fn problem_506_ex2() {
        assert_eq!(
            Solution::find_relative_ranks(vec![10, 3, 8, 9, 4]),
            ["Gold Medal", "5", "Bronze Medal", "Silver Medal", "4"]
        );
    }

    #[test]
    fn problem_506_ex3() {
        assert_eq!(
            Solution::find_relative_ranks(vec![10, 3, 5, 7, 4]),
            ["Gold Medal", "5", "Bronze Medal", "Silver Medal", "4"]
        );
    }

    #[test]
    fn problem_506_ex4() {
        assert_eq!(Solution::find_relative_ranks(vec![10]), ["Gold Medal"]);
    }

    #[test]
    fn problem_506_ex5() {
        assert_eq!(
            Solution::find_relative_ranks(vec![1, 3, 10]),
            ["Bronze Medal", "Silver Medal", "Gold Medal"]
        );
    }
}
